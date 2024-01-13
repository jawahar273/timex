const rewrites = () => {
    
    if(!process.env.NODE_ENV === 'production') {    
        return []    
    } else {
        return [
            {
                source: '/api/:path*',
                destination: 'https://timex.up.railway.app/api/:path*'
            }
        ]
    }
}

/** @type {import('next').NextConfig} */
const nextConfig = {
    experimental: {
        typedRoutes: true,
    },
    rewrites: rewrites,
};

module.exports = nextConfig;
