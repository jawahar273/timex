
/** @type {import('next').NextConfig} */
const nextConfig = {
    experimental: {
        typedRoutes: true,
    },
    output: 'export',
    assetPrefix: process.env.NEXT_PUBLIC_BASE_PATH || '/timex',
    basePath: process.env.NEXT_PUBLIC_BASE_PATH || '/timex',
    // +  assetPrefix = process.env.NEXT_PUBLIC_BASE_PATH || ''
};

console.log(process.env.NEXT_PUBLIC_BASE_PATH)

module.exports = nextConfig;
