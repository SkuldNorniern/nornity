<!---->
<!-- <script lang="ts" setup> -->
<!-- const { data } = await useAsyncData('page-data', () => queryContent($route.params.slug).findOne()) -->
<!-- </script> -->
<!---->
<!-- <template> -->
<!--   <main> -->
<!--     <ContentRenderer :value="data"> -->
<!--       <h1>{{ data.title }}</h1> -->
<!--       <ContentRendererMarkdown :value="data" /> -->
<!--     </ContentRenderer> -->
<!--   </main> -->
<!-- </template> -->

<template>
  <div v-if="data" class="article-container">
    <h1>{{ data.title }}</h1>
    <div class="article-meta">
      <div class="author-info">
        <img :src="data.authorAvatar" alt="Author's avatar" class="author-avatar">
        <span class="author-name">By {{ data.author }}</span>
      </div>
      <span class="reading-time">{{ readingTime }} min read</span>
    </div>
    <ContentDoc :document="data" />
  </div>
  <div v-else class="loading-container">
    <div class="loading-spinner"></div>
    <p>Loading article...</p>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref, watch, nextTick } from 'vue';
import { useAsyncData } from 'nuxt/app';
import { useRoute } from 'vue-router';

const route = useRoute();
const { data, error } = await useAsyncData('page-data', () => queryContent(route.params.slug).findOne());
const contentLoaded = ref(false);
const readingTime = ref('');

watch(data, async () => {
  if (data.value?.body) {
    const extractText = (nodes) => {
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
</script>

<style scoped>
:root {
  --background-color: #0f2027;
  --text-color: #c9d1d9;
  --primary-color: #0070f3;
  --secondary-color: #586069;
  --link-color: #0366d6;
  --link-hover-color: #ff6347;
  --border-color: #e1e4e8;
  --shadow-color: rgba(0, 0, 0, 0.2);
}

.article-container {
  max-width: 800px;
  margin: 60px auto;
  padding: 2.5rem;
  background: var(--background-color);
  color: var(--text-color);
  box-shadow: 0 4px 15px var(--shadow-color);
  border-radius: 12px;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.article-container:hover {
  transform: translateY(-5px);
  box-shadow: 0 6px 20px var(--shadow-color);
}

.article-meta, .author-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
}

.author-avatar {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  margin-right: 10px;
  border: 2px solid var(--primary-color);
}

.author-name, .reading-time {
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
