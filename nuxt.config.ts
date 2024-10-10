// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: {
    enabled: false,

    timeline: {
      enabled: true
    }
  },

  content: {
    // Other Nuxt Content configurations...
    highlight: {
      theme: 'github-dark', // or any other Shiki theme you prefer
      // Optionally, specify additional languages you need highlighting for
      langs: ['rust', 'python', 'c', 'cpp']
    }
  },

  modules: ['@nuxt/content', '@nuxt/image', '@nuxtjs/seo'],

  nitro: {
    prerender: {
      routes: ["/rss.xml"],
    },
  },

  vite: {
    server: {
      hmr: false,
    },
  },
  telemetry: false,
  compatibilityDate: '2024-09-25',
  seo: {
    // Global SEO configuration
    baseUrl: 'https://nornity.com', // Replace with your actual domain
    name: 'Nornity',
    title: 'Nornity - AI, Rust, and Everything in Between',
    description: 'Nornity is a place for sharing thoughts on AI, Rust, and everything in between.',
  },
  sitemap: {
    hostname: 'https://nornity.com', // Replace with your actual domain
  },
})