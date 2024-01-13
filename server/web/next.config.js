const rewrites = () => {
    return [
        {
            source: '/api/:path*',
            destination: 'https://timex.up.railway.app/:path*'
        }
    ]
}

/** @type {import('next').NextConfig} */
const nextConfig = {
    experimental: {
        typedRoutes: true,
    },
    rewrites: rewrites,
};

module.exports = nextConfig;
