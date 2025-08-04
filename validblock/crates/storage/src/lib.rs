#![forbid(unsafe_code)]

use rusqlite::{params, Connection, OptionalExtension};
use validblock_types::{AnchorRecord, Digest256, VBError};

#[derive(Debug)]
pub struct AnchorRepo {
  conn: Connection,
}

impl AnchorRepo {
  /// Open or create anchors.db in cwd
  pub fn new(path: Option<&str>) -> Result<Self, VBError> {
    let db_path = path.unwrap_or("anchors.db");
    let conn = Connection::open(db_path).map_err(|e| VBError::Db(e.to_string()))?;
    Self::init_schema(&conn)?;
    Ok(Self { conn })
  }

  /// For in-memory DB (for tests)
  pub fn memory() -> Result<Self, VBError> {
    let conn = Connection::open_in_memory().map_err(|e| VBError::Db(e.to_string()))?;
    Self::init_schema(&conn)?;
    Ok(Self { conn })
  }

  fn init_schema(conn: &Connection) -> Result<(), VBError> {
    conn.execute(
      "CREATE TABLE IF NOT EXISTS anchors (
        digest BLOB PRIMARY KEY,
        ts INTEGER NOT NULL,
        memo BLOB NULL,
        txid TEXT NULL
      )",
      [],
    ).map_err(|e| VBError::Db(e.to_string()))?;
    Ok(())
  }

  /// Upsert anchor record
  pub fn insert(&self, rec: &AnchorRecord) -> Result<(), VBError> {
    let res = self.conn.execute(
      "INSERT INTO anchors (digest, ts, memo, txid) VALUES (?1, ?2, ?3, ?4)",
      params![&rec.digest.0, rec.ts, &rec.memo, &rec.txid],
    );
    match res {
      Ok(_) => Ok(()),
      Err(rusqlite::Error::SqliteFailure(e, _)) if e.extended_code == 2067 || e.extended_code == 1555 => {
          // 2067: SQLITE_CONSTRAINT_UNIQUE, 1555: SQLITE_CONSTRAINT_PRIMARYKEY
          Err(VBError::DbDuplicate)
      }
      Err(e) => Err(VBError::Db(e.to_string())),
    }
  }

  /// Get anchor by digest
  pub fn get(&self, digest: &Digest256) -> Result<Option<AnchorRecord>, VBError> {
    self.conn
      .query_row(
        "SELECT digest, ts, memo, txid FROM anchors WHERE digest = ?1",
        params![&digest.0],
        |row| {
            Ok(AnchorRecord {
                digest: Digest256(row.get(0)?),
                ts: row.get(1)?,
                memo: row.get(2)?,
                txid: row.get(3)?,
            })
        },
      )
      .optional()
      .map_err(|e| VBError::Db(e.to_string()))
  }

  /// Get all anchors
  pub fn all(&self) -> Result<Vec<AnchorRecord>, VBError> {
    let mut stmt = self.conn.prepare("SELECT digest, ts, memo, txid FROM anchors").map_err(|e| VBError::Db(e.to_string()))?;
    let rows = stmt.query_map([], |row| {
      Ok(AnchorRecord {
        digest: Digest256(row.get(0)?),
        ts: row.get(1)?,
        memo: row.get(2)?,
        txid: row.get(3)?,
      })
    }).map_err(|e| VBError::Db(e.to_string()))?;
    let mut out = Vec::new();
    for r in rows {
      out.push(r.map_err(|e| VBError::Db(e.to_string()))?);
    }
    Ok(out)
  }

  /// WAL checkpoint (stub ok)
  pub fn checkpoint(&self) -> Result<(), VBError> {
    self.conn.execute("PRAGMA wal_checkpoint(TRUNCATE)", []).map_err(|e| VBError::Db(e.to_string()))?;
    Ok(())
  }

  /// Check if a digest exists in the DB
  pub fn exists_digest(&self, digest: &Digest256) -> Result<bool, VBError> {
    let mut stmt = self.conn.prepare("SELECT 1 FROM anchors WHERE digest = ?1 LIMIT 1")
        .map_err(|e| VBError::Db(e.to_string()))?;

    let mut rows = stmt.query(params![&digest.0])
        .map_err(|e| VBError::Db(e.to_string()))?;

        Ok(rows.next().map_err(|e| VBError::Db(e.to_string()))?.is_some())
  }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
  use super::*;
  use validblock_types::Digest256;

  #[test]
  fn test_insert_get() {
    let repo = AnchorRepo::memory().unwrap();
    let rec = AnchorRecord {
      digest: Digest256([1; 32]),
      ts: 42,
      memo: Some(vec![1, 2, 3]),
      txid: Some("txid123".to_string()),
    };
    repo.insert(&rec).unwrap();
    let got = repo.get(&rec.digest).unwrap().unwrap();
    assert_eq!(got, rec);
  }

  #[test]
  fn test_duplicate_insert() {
    let repo = AnchorRepo::memory().unwrap();
    let rec = AnchorRecord {
      digest: Digest256([2; 32]),
      ts: 99,
      memo: None,
      txid: None,
    };
    repo.insert(&rec).unwrap();
    let err = repo.insert(&rec).unwrap_err();
    match err {
      VBError::DbDuplicate => (),
      _ => panic!("Expected DbDuplicate error"),
    }
  }
} 