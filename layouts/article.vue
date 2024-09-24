```vue:layouts/article.vue
<template>
  <div id="article-layout">
    <nav class="top-nav">
      <a href="/">Home</a>
      <a href="/about">About</a>
    </nav>
    <main>
      <slot />
    </main>
    <footer>
      <p>© 2024 SkuldNorniern. All rights reserved.</p>
      <p>Follow on <a href="https://github.com/SkuldNorniern" target="_blank">GitHub</a>.</p>
    </footer>
  </div>
</template>

<script lang="ts">
export default {
  name: 'ArticleLayout',
};
</script>

<style scoped>
#article-layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background-color: var(--background-color);
  color: var(--text-color);
}

.top-nav {
  background-color: #24292e;
  color: white;
  display: flex;
  justify-content: space-between;
  padding: 1rem;
}

.top-nav a {
  color: white;
  text-decoration: none;
  padding: 0 1rem;
  transition: color 0.3s;
}

.top-nav a:hover {
  color: #00ffff;
}

main {
  flex-grow: 1;
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

footer {
  background-color: #24292e;
  color: #9da5b4;
  text-align: center;
  padding: 1rem;
}

footer a {
  color: #9da5b4;
  text-decoration: none;
  transition: color 0.3s;
}

footer a:hover {
  color: white;
}
</style>
