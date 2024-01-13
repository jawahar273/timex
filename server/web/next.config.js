const path = require('path');

const rewrites = () => {
    const API_DOMAIN = process.env.API_DOMAIN || 'http://localhost:8300'

        return [
            {
                source: '/api/:path*',
                destination: path.join(API_DOMAIN, '/api/:path*')
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
