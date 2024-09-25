<template>
  <div class="search-results">
    <div v-for="article in articles" :key="article.slug" class="result-item">
      <nuxt-link :to="`/articles/${article.slug}`">{{ article.title }}</nuxt-link>
      
    </div>
  </div>
</template>

<script>
export default {
  props: {
    articles: {
      type: Array,
      required: true,
    },
  },
};
</script>

<style scoped>
.search-results {
  /* Styling for search results */
}

.result-item {
  margin-bottom: 10px;
  /* Additional styling */
}
</style>

