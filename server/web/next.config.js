
const rewrites = () => {
    const API_DOMAIN = process.env.API_DOMAIN || 'http://localhost:8300'
console.log("api path", new URL ('/api/:path*', API_DOMAIN).href)
        return [
            {
                source: '/api/:path*',
                destination: new URL ('/api/:path*', API_DOMAIN).href
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
