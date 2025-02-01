/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.rs'],
  theme: {
    extend: {
      colors: {
        'rhs-orange-100': '#F28705',
        'rhs-orange-200': '#D97904',
        'rhs-orange-300': '#BF6B04',
        'rhs-orange-400': '#A64F03',
        'rhs-orange-500': '#732C02',
        'rhs-orange-600': '#5a2202',
        'rhs-orange-700': '#411901',
        'rhs-orange-800': '#280f01',
        'rhs-orange-900': '#0f0600',
      }
    }
  },
  plugins: []
};
