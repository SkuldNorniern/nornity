<template>
  <div class="search-results">
    <div v-for="article in articles" :key="article.slug" class="result-item">
      <nuxt-link :to="`/articles/${article.slug}`">{{ article.title }}</nuxt-link>
      <!-- Add more article details if needed -->
    </div>
  </div>
</template>

<script>
export default {
  props: {
    articles: Array,
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

