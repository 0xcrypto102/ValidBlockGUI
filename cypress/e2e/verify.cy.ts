describe('Anchor flow', () => {
    it('anchors locally', () => {
      cy.visit('/');
      cy.contains('Anchor').click();
      cy.get('input[type=file]').selectFile('cypress/fixtures/test.txt');
      cy.contains('Anchor Locally').click();
      cy.contains('Anchored with digest').should('exist');
    });
});