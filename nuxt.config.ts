// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: {
    enabled: true,

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
 modules: ['@nuxt/content']
})
