<template>
  <div v-if="data" class="article-container">
    <header class="article-header">
      <h1 class="article-title">{{ data.title }}</h1>
      <div class="article-meta">
        <div class="author-info">
          <img :src="data.authorAvatar || '/default/thumbnail.png'" alt="Author's avatar" class="author-avatar">
          <span class="author-name">By {{ data.author }}</span>
        </div>
        <div class="article-details">
          <div class="tags">
            <span v-for="tag in data.tag" class="tag-badge">{{ tag }}</span>
          </div>
          <span class="article-date">Published on:  {{ formatDate(data.date) }}</span>
          <span class="reading-time">{{ readingTime }} min read</span>
        </div>
      </div>
    </header>
    <ContentRenderer :value="data" />
  </div>
  <div v-else-if="error">
    <h1>Error</h1>
    <p>{{ error.message }}</p>
    <p>Slug: {{ slug }}</p>
  </div>
  <div v-else class="loading-container">
    <div class="loading-spinner"></div>
    <p>Loading article...</p>
  </div>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue';
import { useAsyncData } from 'nuxt/app';
import { useRoute } from 'vue-router';
import { useSeoMeta } from '@unhead/vue';

const route = useRoute();
const slug = Array.isArray(route.params.slug) ? route.params.slug.join('/') : route.params.slug;

console.log('Current slug:', slug); // Debug log

const { data, error } = await useAsyncData('page-data', async () => {
  console.log('Querying content for slug:', slug); // Debug log
  const article = await queryContent('articles')
    .where({ slug: slug })
    .findOne();

  console.log('Query result:', article); // Debug log

  if (!article) {
    throw new Error('Article not found');
  }

  return article;
});

const contentLoaded = ref(false);
const readingTime = ref('');

watch(data, async () => {
  if (data.value?.body) {
    const extractText = (nodes: any[]): string => {
      if (!nodes || !Array.isArray(nodes)) {
        return '';
      }

      return nodes.reduce((acc, node) => {
        if (node.type === 'element' && node.children) {
          return acc + ' ' + extractText(node.children);
        } else if (node.type === 'text') {
          return acc + ' ' + node.value;
        }
        return acc;
      }, '');
    };

    const allText = extractText(data.value.body.children);
    const words = allText.trim().split(/\s+/).length;
    const minutes = Math.ceil(words / 200);
    readingTime.value = minutes.toString();

    contentLoaded.value = true;
  }
}, { immediate: true, deep: true });

const formatDate = (dateString: string) => {
  const options: Intl.DateTimeFormatOptions = { year: 'numeric', month: 'long', day: 'numeric' };
  return new Date(dateString).toLocaleDateString(undefined, options);
};
useSeoMeta({
  title: () => data.value?.title ?? 'Article',
  ogTitle: () => data.value?.title ?? 'Article',
  description: () => data.value?.description ?? '',
  ogDescription: () => data.value?.description ?? '',
  ogImage: () => data.value?.image || '/default-og-image.jpg',
  twitterCard: 'summary_large_image',
});
</script>

<style scoped>
:root {
  --background-color: #f9f9f9;
  --text-color: #4a4a4a;
  --primary-color: #5a67d8;
  --secondary-color: #718096;
  --link-color: #2b6cb0;
  --link-hover-color: #2c5282;
  --border-color: #e2e8f0;
  --shadow-color: rgba(0, 0, 0, 0.05);
}

.article-container {
  max-width: 800px;
  margin: 60px auto;
  padding: 2.5rem;
  background: var(--background-color);
  color: var(--text-color);
  box-shadow: 0 2px 10px var(--shadow-color);
  border-radius: 8px;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.article-header {
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 1rem;
  margin-bottom: 2rem;
}

.article-title {
  font-size: 2rem;
  margin-bottom: 1rem;
  color: var(--primary-color);
}

.article-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.author-info {
  display: flex;
  align-items: center;
}

.author-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  margin-right: 10px;
  border: 1px solid var(--primary-color);
}

.author-name {
  font-size: 0.9rem;
  color: var(--secondary-color);
}

.article-details {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
}

.tags {
  margin-bottom: 0.5rem;
}

.tag-badge {
  display: inline-block;
  background-color: var(--primary-color);
  color: #fff;
  padding: 0.2rem 0.5rem;
  margin: 0.2rem 0.3rem;
  border-radius: 8px;
  font-size: 0.75rem;
  font-weight: 500;
  transition: all 0.3s ease;
}

.tag-badge:hover {
  background-color: var(--link-hover-color);
  transform: translateY(-2px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.article-date {
  font-size: 0.9rem;
  color: var(--secondary-color);
  margin-bottom: 0.5rem;
}

.reading-time {
  font-size: 0.9rem;
  color: var(--secondary-color);
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  color: var(--text-color);
}

.loading-spinner {
  border: 4px solid var(--secondary-color);
  border-top: 4px solid var(--primary-color);
  border-radius: 50%;
  width: 40px;
  height: 40px;
  animation: spin 1s linear infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

h1, h2, h3, h4, h5, h6 {
  color: var(--text-color);
  margin-top: 2rem;
  font-weight: 700;
}
</style>