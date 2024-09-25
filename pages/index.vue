<template>
  <div class="container">
    <!-- Navigation -->
    <nav class="navigation">
      <div class="search-container">
        <input type="text" placeholder="Search articles..." class="search-input" v-model="searchQuery" @input="performSearch" />
        <!-- Floating Search Results Panel -->
        <div class="floating-search-results" v-if="filteredArticles.length">
          <SearchResults :filteredArticles="filteredArticles" :searchQuery="searchQuery"></SearchResults>
        </div>
      </div>
    </nav>

    <!-- Main Content -->
    <main class="main-content">
      <div class="left-column">
        <!-- Featured Article -->
        <section class="featured-article" v-for="lastarticle in latestArticle" :key="latestArticle.slug">
          <div class="floating-orb"></div>
          
          <div class="article-content">
            <div class="article-meta">Latest Article</div>
            <h1 class="article-title">{{ lastarticle.title }}</h1>
            <p>{{ lastarticle.date }}</p>
            <p>{{ truncateDescription(lastarticle.description, 100) }}</p>
            <a :href="`/articles/${lastarticle.slug}`" class="read-more">Read article</a>
          </div>
        </section>

        <!-- About Section (Replaced Trending Posts) -->
        <section class="featured-article about-section">
          <div class="article-content">
            <div class="about-header">
              <img src="https://avatars.githubusercontent.com/u/43695854?v=4" alt="Profile Picture" class="profile-picture" />
              <h1 class="article-title">Skuld Norniern</h1>
            </div>
            
            <div class="about-me">
              <h2>About Me</h2>
              <p>Welcome to my blog! Here you can find information about my dev log, and my story.</p>
              <p>Available Stacks: C/C++, Rust, Go, Python</p>
            </div>

            <div class="social-links">
              <a href="https://github.com/SkuldNorniern" target="_blank">GitHub</a>
              <a href="https://twitter.com/SkuldNorniern" target="_blank">Twitter</a>
              <!-- <a href="https://www.linkedin.com/in//" target="_blank">LinkedIn</a> -->
            </div>
          </div>
          
          <!-- Floating Decorations -->
          <div class="floating-decorations">
            <div class="decoration decoration-1"></div>
            <div class="decoration decoration-2"></div>
            <div class="decoration decoration-3"></div>
          </div>
        </section>
      </div>

      <div class="right-column">
        <!-- Recommended Articles -->
        <section class="recommended-articles">
          <h2 class="section-title">Recommended</h2>
          <div class="article-grid" ref="articleGrid">
            <!-- Article cards go here -->
            <article class="article-card" v-for="article in displayedArticles" :key="article.slug">
              <div class="article-content">
                <h3>{{ article.title }}</h3>
                <p class="article-date">{{ article.date }}</p>
                <p class="article-description">{{ truncateDescription(article.description, 100) }}</p>
              </div>
              <a :href="`/articles/${article.slug}`" class="read-more-light">Read article</a>
            </article>
          </div>
          <button v-if="displayedArticles.length < articles.length" @click="loadMoreArticles" class="load-more-button">
            Load More
          </button>
        </section>

        
      </div>
      
    </main>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, watch, onMounted, computed } from 'vue';
import FeaturedArticles from '@/components/FeaturedArticles.vue';
import FeaturedProducts from '@/components/FeaturedProducts.vue';
import SearchResults from '@/components/SearchResults.vue'; // Correct import

export default defineComponent({
  components: {
    FeaturedArticles,
    FeaturedProducts,
    SearchResults, // Correct registration
  },
  setup() {
    const searchQuery = ref('');
    const articles = ref<Array<{ title: string; slug: string; date: string; [key: string]: any }>>([]);
    const latestArticle = ref<Array<{ title: string; slug: string; date: string; [key: string]: any }>>([]);
    const filteredArticles = ref<Array<{ title: string; [key: string]: any }>>([]);
    const featuredArticles = ref<Array<{ title: string; [key: string]: any }>>([]);
    const featuredProducts = ref<Array<{ name: string; [key: string]: any }>>([]);
    const isLoading = ref(false);
    const articleGrid = ref(null);
    const displayedArticles = ref([]);
    const articlesPerPage = ref(6); // Default number of articles to display

    const fetchArticles = async () => {
      try {
        isLoading.value = true;
        const fetchedArticles = await queryContent('articles')
          .where({ _extension: 'md' })
          .find();
        articles.value = fetchedArticles;
        console.log('Article List:', articles.value);
      } catch (error) {
        console.error('Error fetching articles:', error);
      } finally {
        isLoading.value = false;
      }
    };

    const LatestArticle = async () => {
      try {
        isLoading.value = true;
        const fetchedArticles = await queryContent('articles')
          .where({ _extension: 'md' })
          .sort({ date: -1 })
          .limit(1)
          .find();
        latestArticle.value = fetchedArticles;
        console.log('Latest Article:', latestArticle.value);
      } catch (error) {
        console.error('Error fetching latest article:', error);
      } finally {
        isLoading.value = false;
      }
    };

    const fetchFeaturedArticles = async () => {
      try {
        isLoading.value = true;
        featuredArticles.value = await queryContent('featured-articles').find();
        console.log('Featured Articles:', featuredArticles.value);
      } catch (error) {
        console.error('Error fetching featured articles:', error);
      } finally {
        isLoading.value = false;
      }
    };

    const fetchFeaturedProducts = async () => {
      try {
        isLoading.value = true;
        featuredProducts.value = await queryContent('featured-products').find();
        console.log('Featured Products:', featuredProducts.value);
      } catch (error) {
        console.error('Error fetching featured products:', error);
      } finally {
        isLoading.value = false;
      }
    };

    const searchArticles = () => {
      if (searchQuery.value.trim() === '') {
        filteredArticles.value = [];
        return;
      }
      console.log('Searching for:', searchQuery.value);
      console.log('Articles:', articles.value);
      filteredArticles.value = articles.value.filter((article) =>
        article.title.toLowerCase().includes(searchQuery.value.toLowerCase())
      );
      console.log('Filtered articles:', filteredArticles.value);
    };

    const performSearch = () => {
      searchArticles();
    };

    const updateDisplayedArticles = () => {
      if (!articleGrid.value || articles.value.length === 0) return;

      const gridHeight = articleGrid.value.clientHeight;
      const articleHeight = 150; // Approximate height of each article card
      const columns = Math.floor(articleGrid.value.clientWidth / 250); // Assuming 250px min-width for cards
      const rows = Math.floor(gridHeight / articleHeight);
      
      const initialCount = columns * rows;
      displayedArticles.value = articles.value.slice(0, initialCount);
      console.log('Displayed articles:', displayedArticles.value); // Add this line for debugging
    };

    const loadMoreArticles = () => {
      const currentLength = displayedArticles.value.length;
      const newArticles = articles.value.slice(currentLength, currentLength + articlesPerPage.value);
      displayedArticles.value = [...displayedArticles.value, ...newArticles];
    };

    watch(searchQuery, searchArticles, { immediate: true });

    onMounted(async () => {
      await Promise.all([
        fetchArticles(),
        LatestArticle(),
        fetchFeaturedArticles(),
        fetchFeaturedProducts(),
      ]);

      updateDisplayedArticles();
      window.addEventListener('resize', updateDisplayedArticles);
    });

    const truncateDescription = (description: string, maxLength: number) => {
      if (description.length <= maxLength) {
        return description;
      }
      return description.slice(0, maxLength) + '...';
    };

    return {
      searchQuery,
      articles,
      latestArticle,
      filteredArticles,
      featuredArticles,
      featuredProducts,
      isLoading,
      performSearch,
      truncateDescription,
      articleGrid,
      displayedArticles,
      loadMoreArticles,
    };
  },
});
</script>

<style scoped>
:root {
  --primary-color: #8a2be2;
  --secondary-color: #f0f0f0;
  --background-color: #ffffff;
  --text-color: #333333;
  --accent-color: #ff4081;
  --font-family: 'Inter', sans-serif;
}

.container {
  font-family: var(--font-family);
  background-color: var(--background-color);
  color: var(--text-color);
  min-height: 100vh;
  padding: 20px;
  /* margin-bottom: 150px; */
  box-sizing: border-box;
}

.navigation {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20px 0;
}

.search-container {
  position: relative;
  width: 100%;
  max-width: 800px; /* Match the width in the image */
  display: flex;
  justify-content: center;
}

.search-input {
  width: 100%;
  max-width: 450px;
  padding: 12px 20px;
  border: 2px solid var(--primary-color);
  border-radius: 30px;
  font-size: 16px;
  transition: all 0.3s ease;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.search-input:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(138, 43, 226, 0.3);
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0% {
    box-shadow: 0 0 0 0 rgba(138, 43, 226, 0.4);
  }
  70% {
    box-shadow: 0 0 0 10px rgba(138, 43, 226, 0);
  }
  100% {
    box-shadow: 0 0 0 0 rgba(138, 43, 226, 0);
  }
}

.main-content {
  display: grid;
  grid-template-columns: 1fr 2fr;
  gap: 40px;
  position: relative;
}

.left-column {
  display: flex;
  flex-direction: column;
  gap: 30px;
}

.right-column {
  /* display: grid; */
  position: relative;
  /* flex-direction: column; */
}

.featured-article {
  background: linear-gradient(135deg, #8a2be2, #4a0e8f);
  color: white;
  padding: 40px;
  border-radius: 20px;
  position: relative;
  overflow: hidden;
}

.article-content {
  position: relative;
  z-index: 2;
}

.article-meta {
  font-size: 12px;
  text-transform: uppercase;
  margin-bottom: 10px;
}

.article-title {
  font-size: 28px;
  font-weight: bold;
  margin-bottom: 20px;
}

.read-more {
  display: inline-block;
  color: white;
  text-decoration: none;
  padding: 8px 16px;
  border: 1px solid white;
  border-radius: 20px;
  font-size: 14px;
}

.read-more-light {
  display: inline-block;
  color: black;
  text-decoration: none;
  padding: 8px 16px;
  border: 1px solid black;
  border-radius: 20px;
  font-size: 14px;
}

.floating-orb {
  position: absolute;
  width: 150px;
  height: 150px;
  background: radial-gradient(circle, #b19cd9, #8a2be2);
  border-radius: 50%;
  filter: blur(20px);
  opacity: 0.7;
  z-index: 1;
  animation: float 20s ease-in-out infinite;
}

@keyframes float {
  0%, 100% { transform: translate(0, 0); }
  25% { transform: translate(50%, 25%); }
  50% { transform: translate(0, 50%); }
  75% { transform: translate(-50%, 25%); }
}

.recommended-articles {
  background: linear-gradient(135deg, #ffffff, #cccccc);;
  color: black;
  border-radius: 20px;
  padding: 30px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  min-height: 400px; /* Add this line to ensure minimum height */
}

.article-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  grid-template-rows: repeat(auto-fill, minmax(220px, 1fr));
  gap: 20px;
  row-gap: 60px;
  padding: 10px;
  height: 60vh; 
  overflow-y: auto;
  padding-right: 15px;
  min-height: 300px; /* Add this line to ensure minimum height */
}

.article-card {
  background-color: #e6f3e6;
  border-radius: 10px;
  padding: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  transition: transform 0.3s ease, box-shadow 0.3s ease;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  height: 100%;
}

.article-content {
  flex-grow: 1;
}

.article-card h3 {
  font-size: 18px;
  margin-bottom: 10px;
  color: #333;
  
}

.article-date {
  font-size: 14px;
  color: #666;
  margin-bottom: 10px;
}

.article-description {
  font-size: 14px;
  color: #444;
  margin-bottom: 15px;
}

.read-more-light {
  align-self: flex-start;
  background-color: transparent;
  color: black;
  text-decoration: none;
  padding: 8px 16px;
  border-radius: 20px;
  font-size: 14px;
  transition: background-color 0.3s ease;
  margin-top: auto;
}

.read-more-light:hover {
  background-color: #45a049;
}

.about {
  font-family: var(--font-family);
  color: var(--text-color);
  background-color: var(--background-color);
  min-height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}

.profile-card {
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(10px);
  border-radius: 20px;
  padding: 3rem;
  box-shadow: 0 8px 32px rgba(0, 255, 255, 0.1);
  text-align: left;
  transition: all 0.5s ease;
  max-width: 600px;
  width: 100%;
}

.profile-card:hover {
  box-shadow: 0 16px 64px rgba(0, 255, 255, 0.2);
}

.profile-card h1 {
  color: var(--primary-color);
  font-size: 2.5rem;
  margin-bottom: 1rem;
  text-transform: uppercase;
  letter-spacing: 2px;
}

.profile-card h2 {
  color: var(--primary-color);
  font-size: 1.8rem;
  margin-bottom: 0.5rem;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.profile-card p {
  margin-bottom: 1rem;
  line-height: 1.6;
  font-size: 1.1rem;
}

.social-links {
  display: flex;
  justify-content: start;
  gap: 20px;
  margin-bottom: 2rem;
}

.social-links a {
  color: var(--primary-color);
  text-decoration: none;
  font-size: 1.2rem;
  transition: color 0.3s ease;
}

.social-links a:hover {
  color: #ffffff;
}

.about-me {
  margin-top: 20px;
}

.about-me h2 {
  font-size: 22px;
  margin-bottom: 10px;
}

.social-links {
  margin-top: 20px;
}

.about-header {
  display: flex;
  align-items: center;
  gap: 20px;
  margin-bottom: 20px;
}

.profile-picture {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  border: 3px solid white;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.about-section {
  background: linear-gradient(135deg, #ff5f7a, #d97bfe);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  padding: 40px;
  border-radius: 20px;
  position: relative;
  overflow: hidden;
}

.about-section .article-content {
  position: relative;
  z-index: 2;
}

.about-section .article-meta {
  font-size: 12px;
  text-transform: uppercase;
  margin-bottom: 10px;
}

.about-section .article-title {
  font-size: 28px;
  font-weight: bold;
  margin: 0;
}

.about-section .social-links a {
  color: white;
  text-decoration: none;
  padding: 8px 16px;
  border: 1px solid white;
  border-radius: 20px;
  font-size: 14px;
  transition: background-color 0.3s ease;
}

.about-section .social-links a:hover {
  background-color: rgba(255, 255, 255, 0.2);
}

.floating-decorations {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  z-index: 1;
}

.decoration {
  position: absolute;
  border-radius: 50%;
  opacity: 0.6;
  animation: float 20s ease-in-out infinite;
}

.decoration-1 {
  width: 100px;
  height: 100px;
  background: rgba(255, 255, 255, 0.3);
  top: 10%;
  left: 20%;
}

.decoration-2 {
  width: 150px;
  height: 150px;
  background: rgba(255, 255, 255, 0.2);
  top: 50%;
  left: 70%;
}

.decoration-3 {
  width: 80px;
  height: 80px;
  background: rgba(255, 255, 255, 0.4);
  top: 80%;
  left: 30%;
}

.trending-list {
  list-style-type: none;
  padding: 0;
}

.trending-list li {
  margin-bottom: 10px;
}

.trending-list a {
  color: var(--text-color);
  text-decoration: none;
  font-weight: 500;
  transition: color 0.3s ease;
}

.trending-list a:hover {
  color: var(--primary-color);
}

/* Add responsive styles as needed */
@media (max-width: 768px) {
  .main-content {
    grid-template-columns: 1fr;
  }

  .left-column,
  .right-column {
    gap: 20px;
  }

  .floating-orb {
    width: 100px;
    height: 100px;
  }
}
.floating-search-results {
  position: absolute;
  margin-top: 50px;
  /* top: 60%; */
  /* left: 50%; */
  /* transform: translate(-50%, -50%); */
  display: flex;
  justify-content: center;
  background-color: white;
  border-radius: 20px;
  padding: 20px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  z-index: 1000;
}

.load-more-button {
  display: block;
  margin: 20px auto 0;
  padding: 10px 20px;
  background-color: var(--primary-color);
  color: white;
  border: none;
  border-radius: 20px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.load-more-button:hover {
  background-color: #6a1cb7;
}
</style>
