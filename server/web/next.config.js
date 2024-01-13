const rewrites = () => {
    
    if(process.env.NODE_ENV === 'production') {        
        return [
            {
                source: '/api/:path*',
                destination: 'https://timex.up.railway.app/api/:path*'
            }
        ]
    } else {
        return []
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
