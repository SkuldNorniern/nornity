<template>
  <div class="search-results">
    <h3 v-if="filteredArticles.length">Search Results</h3>
    <div v-else class="no-results">
      <p>No articles found for "{{ searchQuery }}"</p>
    </div>
    <div class="articles-list">
      <div v-for="article in filteredArticles" :key="article.id" class="article-item">
        <div class="article-content">
          <h4>{{ article.title }}</h4>
          <p class="article-date">{{ formatDate(article.date) }}</p>
          <p class="article-snippet">
            {{ article.description || (article.content?.slice(0, 100) || 'No description available') }}...
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
  color: black
}

.no-results {
  text-align: center;
  color: var(--secondary-color);
  font-size: 1.1em;
  margin-top: 20px;
}

.articles-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
  margin-top: 20px;
}

.article-item {
  background-color: #ffffff;
  border-radius: 10px;
  overflow: hidden;
  transition: box-shadow 0.3s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.article-item:hover {
  box-shadow: 0 4px 8px rgba(138, 43, 226, 0.2);
}

.article-content {
  padding: 20px;
}

.article-content h4 {
  margin: 0 0 10px 0;
  color: var(--primary-color);
  font-size: 1.2em;
}

.article-date {
  font-size: 0.9em;
  color: #666666;
  margin-bottom: 10px;
}

.article-snippet {
  color: #333333;
  margin-bottom: 15px;
  line-height: 1.4;
}

.read-more {
  display: inline-block;
  padding: 6px 12px;
  color: var(--primary-color);
  border: 1px solid var(--primary-color);
  border-radius: 20px;
  text-decoration: none;
  font-size: 0.9em;
  transition: background-color 0.3s, color 0.3s;
}

.read-more:hover {
  background-color: var(--primary-color);
  color: white;
}
</style>

