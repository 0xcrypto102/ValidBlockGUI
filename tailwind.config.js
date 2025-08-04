/** @type {import('tailwindcss').Config} */
export default {
    darkMode: 'class',         // 👈 class‑based toggle
    content: ['./index.html', './src/**/*.{ts,tsx}'],
    theme: {
      extend: {
        colors: {
          'bg':            'var(--c-bg)',
          'panel':         'var(--c-panel)',
          'accent-primary':'var(--c-accent)',
          'text-primary':  'var(--c-text)',
        },
      },
    },
    plugins: [],
};