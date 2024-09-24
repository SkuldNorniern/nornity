<script setup lang="ts">
const articles = await queryContent('articles')
  .sort({ title: 1 })
  .find();
</script>

<template>
  <section class="featured-articles">
    <h2>Featured Articles</h2>
    <div class="articles-grid">
      <div class="article-card" v-for="article in articles" :key="article.slug">
        <nuxt-link :to="`/articles/${article.slug}`">
          <div class="card-image">
            <img v-if="article.image" :src="article.image" alt="" />
          </div>
          <div class="card-content">
            <h3>{{ article.title }}</h3>
          </div>
        </nuxt-link>
      </div>
    </div>
  </section>
</template>

<style scoped>
.featured-articles {
  padding: 60px 0;
}

.featured-articles h2 {
  font-size: 2rem;
  font-weight: 600;
  text-align: center;
  color: #1d1d1f;
  margin-bottom: 40px;
}

.articles-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  justify-content: center;
}

.article-card {
  background-color: #fff;
  border-radius: 12px;
  overflow: hidden;
  width: 300px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  transition: transform 0.3s;
}

.article-card:hover {
  transform: translateY(-5px);
}

.card-image img {
  width: 100%;
  height: auto;
  display: block;
}

.card-content {
  padding: 20px;
}

.card-content h3 {
  font-size: 1.25rem;
  font-weight: 500;
  color: #1d1d1f;
  margin: 0;
}

@media (max-width: 768px) {
  .article-card {
    width: 100%;
  }
}
</style>