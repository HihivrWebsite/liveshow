/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './src/**/*.{vue,js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        background: '#F7F1DF',
        primary: '#F6B100',
        accent: '#FF6B9D',
        'text-main': '#5D4B24',
        'text-secondary': '#8E7B50',
        card: '#FFF3D0',
      },
      borderRadius: {
        button: '999px',
        input: '24px',
        card: '32px',
        dialog: '40px',
      },
      boxShadow: {
        default: '0 8px 24px rgba(0, 0, 0, 0.04)',
        hover: '0 12px 32px rgba(0, 0, 0, 0.08)',
        dialog: '0 20px 60px rgba(0, 0, 0, 0.12)',
      },
      backdropBlur: {
        glass: '24px',
      },
      transitionDuration: {
        fast: '0.15s',
        normal: '0.3s',
        slow: '0.5s',
      },
      transitionTimingFunction: {
        out: 'cubic-bezier(0.22, 1, 0.36, 1)',
        'in-out': 'cubic-bezier(0.65, 0, 0.35, 1)',
        spring: 'cubic-bezier(0.34, 1.56, 0.64, 1)',
      },
    },
  },
  plugins: [],
}
