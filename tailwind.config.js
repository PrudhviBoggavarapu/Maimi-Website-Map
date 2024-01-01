/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'], darkMode: 'class', // Enables dark mode
    theme: {
        extend: {
            colors: {
                blue: {
                    DEFAULT: 'var(--color-blue)',
                    light: '#1fb6ff',
                    dark: '#0a85d9', // Example dark variant for blue
                },
                purple: {
                    DEFAULT: 'var(--color-purple)',
                    light: '#7e5bef',
                    dark: '#6244b8', // Dark variant for purple
                },
                pink: {
                    DEFAULT: 'var(--color-pink)',
                    light: '#ff49db',
                    dark: '#db32c1', // Dark variant for pink
                },
                orange: {
                    DEFAULT: 'var(--color-orange)',
                    light: '#ff7849',
                    dark: '#e45b25', // Dark variant for orange
                },
                green: {
                    DEFAULT: 'var(--color-green)',
                    light: '#13ce66',
                    dark: '#0fa855', // Dark variant for green
                },
                yellow: {
                    DEFAULT: 'var(--color-yellow)',
                    light: '#ffc82c',
                    dark: '#e6b000', // Dark variant for yellow
                },
                'gray-dark': {
                    DEFAULT: 'var(--color-gray-dark)',
                    light: '#273444',
                    dark: '#1c262f', // Dark variant for gray-dark
                },
                gray: {
                    DEFAULT: 'var(--color-gray)',
                    light: '#8492a6',
                    dark: '#6b798e', // Dark variant for gray
                },
                'gray-light': {
                    DEFAULT: 'var(--color-gray-light)',
                    light: '#d3dce6',
                    dark: '#b8c2cc', // Dark variant for gray-light
                }
            }
        },
        fontFamily: {
            sans: [
                'Graphik', 'sans-serif'
            ],
            serif: ['Merriweather', 'serif']
        },
        extend: {
            spacing: {
                '128': '32rem',
                '144': '36rem'
            },
            borderRadius: {
                '4xl': '2rem'
            }
        },
        plugins: []
    }
}
