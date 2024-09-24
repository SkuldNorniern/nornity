<template>
  <div class="search-results">
    <h3 v-if="filteredArticles.length">Search Results:</h3>
    <div v-else class="no-results">
      <p>No articles found for "{{ searchQuery }}"</p>
    </div>
    <div class="articles-grid">
      <div v-for="article in filteredArticles" :key="article.id" class="article-card">
        <img 
          :src="article.thumbnail || defaultThumbnail" 
          alt="Article Thumbnail" 
          class="article-image" 
          loading="lazy"
        />
        <div class="article-content">
          <h4>{{ article.title }}</h4>
          <p class="article-date">{{ formatDate(article.date) }}</p>
          <p class="article-snippet">
            {{ article.description || article.content.slice(0, 100) }}...
          </p>
          <NuxtLink :to="`/articles/${article.slug}`" class="read-more">Read More</NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'SearchResults',
  props: {
    filteredArticles: {
      type: Array,
      required: true,
    },
    searchQuery: {
      type: String,
      required: false,
      default: '',
    },
  },
  setup(props) {
    const defaultThumbnail = '/default/thumbnail.png'; 

    const formatDate = (date: string) => {
      const options: Intl.DateTimeFormatOptions = { year: 'numeric', month: 'long', day: 'numeric' };
      return new Date(date).toLocaleDateString(undefined, options);
    };

    return {
      defaultThumbnail,
      formatDate,
    };
  },
});
</script>

<style scoped>
.search-results {
  margin-top: 20px;
  padding: 0 20px;
}

.no-results {
  text-align: center;
  color: var(--secondary-color);
  font-size: 1.2em;
  margin-top: 20px;
}

.articles-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.article-card {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border-radius: 15px;
  overflow: hidden;
  transition: transform 0.3s, box-shadow 0.3s;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.article-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 12px 24px rgba(0, 255, 255, 0.2);
}

.article-image {
  width: 100%;
  height: 180px;
  object-fit: cover;
}

.article-content {
  padding: 15px;
  display: flex;
  flex-direction: column;
  flex-grow: 1;
}

.article-content h4 {
  margin: 0 0 10px 0;
  color: var(--primary-color);
  font-size: 1.25em;
}

.article-date {
  font-size: 0.9em;
  color: #c9d1d9;
  margin-bottom: 10px;
}

.article-snippet {
  flex-grow: 1;
  color: var(--secondary-color);
  margin-bottom: 15px;
}

.read-more {
  align-self: flex-start;
  padding: 8px 16px;
  background-color: var(--primary-color);
  color: var(--background-color);
  border-radius: 20px;
  text-decoration: none;
  transition: background-color 0.3s, transform 0.3s;
}

.read-more:hover {
  background-color: #00cccc;
  transform: translateY(-2px);
}
</style>

