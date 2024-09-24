<script lang="ts" setup>
const { data } = await useAsyncData('page-data', () => queryContent('/articles').findOne())
</script>

<template>
  <div class="container">
    <div class="hero-section">
      <h1>{{ data?.title }}</h1>
    </div>
    <main class="main-content">
      <ContentRenderer v-if="data" :value="data">
        <ContentRendererMarkdown :value="data" />
      </ContentRenderer>
    </main>
  </div>
</template>

<style scoped>
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
}

.hero-section {
  padding: 80px 0;
  text-align: center;
  background-color: #f5f5f7; /* Light gray background */
}

.hero-section h1 {
  font-size: 3rem;
  font-weight: 600;
  color: #1d1d1f; /* Dark gray text */
}

.main-content {
  margin-top: 40px;
  line-height: 1.6;
  color: #1d1d1f;
}

.content-renderer-markdown {
  font-size: 1rem;
}

@media (max-width: 768px) {
  .hero-section {
    padding: 60px 0;
  }

  .hero-section h1 {
    font-size: 2.5rem;
  }
}
</style>

