/** @type {import('next').NextConfig} */
const nextConfig = {
  async rewrites() {
    return [
      {
        source: "/getting.v1.:services/:paths*",
        destination: "http://localhost:9999/getting.v1.:services/:paths*",
      },
    ];
  },
};

export default nextConfig;
