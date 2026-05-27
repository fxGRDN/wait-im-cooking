/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        app: {
          bg: 'rgb(var(--color-bg) / <alpha-value>)',
          'bg-soft': 'rgb(var(--color-bg-soft) / <alpha-value>)',
          surface: 'rgb(var(--color-surface) / <alpha-value>)',
          'surface-strong': 'rgb(var(--color-surface-strong) / <alpha-value>)',
          subtle: 'rgb(var(--color-subtle) / <alpha-value>)',
          border: 'rgb(var(--color-border) / <alpha-value>)',
          text: 'rgb(var(--color-text) / <alpha-value>)',
          muted: 'rgb(var(--color-muted) / <alpha-value>)',
          accent: 'rgb(var(--color-accent) / <alpha-value>)',
          'accent-strong': 'rgb(var(--color-accent-strong) / <alpha-value>)',
        },
      },
    },
  },
  plugins: [],
};
