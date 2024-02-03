
/** @type {import('next').NextConfig} */
const nextConfig = {
    experimental: {
        typedRoutes: true,
    },
    output: 'export',
    assetPrefix: process.env.NEXT_PUBLIC_BASE_PATH || '',
    basePath: process.env.NEXT_PUBLIC_BASE_PATH || '',
    // +  assetPrefix = process.env.NEXT_PUBLIC_BASE_PATH || ''
};

module.exports = nextConfig;
