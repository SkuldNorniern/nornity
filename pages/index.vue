<template>
  <div class="container">
    <!-- Hero Section -->
    <section class="hero-section">
      <div class="hero-content">
        <h1 class="hero-title">Welcome to Nornity</h1>
        <p class="hero-subtitle">Discover the latest articles and explore our products</p>
      </div>
    </section>

    <!-- Content Card -->
    <section class="content-card">
      <!-- Search Bar -->
      <div class="search-bar">
        <input
          type="text"
          v-model="searchQuery"
          placeholder="Search articles..."
          class="search-input"
          aria-label="Search articles"
        />
      </div>

      <!-- Search Results or Featured Content -->
      <div v-if="searchQuery">
        <SearchResults :filteredArticles="filteredArticles" />
      </div>
      <div v-else>
        <!-- Featured Articles -->
        <section class="featured-section">
          <h2 class="section-title">Featured Articles</h2>
          <FeaturedArticles :articles="featuredArticles" />
        </section>

        <!-- Featured Products -->
        <section class="products-section">
          <h2 class="section-title">Featured Products</h2>
          <FeaturedProducts :products="featuredProducts" />
        </section>
      </div>
    </section>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, watch, onMounted } from 'vue';
import FeaturedArticles from '@/components/FeaturedArticles.vue';
import FeaturedProducts from '@/components/FeaturedProducts.vue';
import SearchResults from '@/components/SearchResults.vue';
// import { queryContent } from '@nuxt/content';

export default defineComponent({
  components: {
    FeaturedArticles,
    FeaturedProducts,
    SearchResults,
  },
  setup() {
    const searchQuery = ref('');
    const articles = ref<Array<{ title: string; [key: string]: any }>>([]);
    const filteredArticles = ref<Array<{ title: string; [key: string]: any }>>([]);
    const featuredArticles = ref<Array<{ title: string; [key: string]: any }>>([]);
    const featuredProducts = ref<Array<{ name: string; [key: string]: any }>>([]);
    const isLoading = ref(false);

    const fetchArticles = async () => {
      try {
        isLoading.value = true;
        // Fetch all articles using queryContent
        articles.value = await queryContent('articles').find();
        console.log('All Articles:', articles.value);
      } catch (error) {
        console.error('Error fetching articles:', error);
      } finally {
        isLoading.value = false;
      }
    };

    const fetchFeaturedArticles = async () => {
      try {
        isLoading.value = true;
        // Fetch featured articles
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
        // Fetch featured products
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

    watch(searchQuery, searchArticles, { immediate: true });

    onMounted(async () => {
      await Promise.all([
        fetchArticles(),
        fetchFeaturedArticles(),
        fetchFeaturedProducts(),
      ]);
    });

    return {
      searchQuery,
      articles,
      filteredArticles,
      featuredArticles,
      featuredProducts,
      isLoading,
      performSearch,
    };
  },
});
</script>

<style scoped>
:root {
  --primary-color: #00c9ff;
  --secondary-color: #f0f0f0;
  --background-color: #1a1a1a;
  --accent-color: #ff4081;
  --font-family: 'Roboto', sans-serif;
  --transition-speed: 0.3s;
}

.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  font-family: var(--font-family);
  color: var(--secondary-color);
  background-color: var(--background-color);
  min-height: 100vh;
  padding: 20px;
  box-sizing: border-box;
}

.hero-section {
  width: 100%;
  padding: 120px 20px;
  text-align: center;
  background: linear-gradient(135deg, #1e3c72, #2a5298);
  color: white;
  border-radius: 15px;
  margin-bottom: 50px;
  box-shadow: 0 12px 24px rgba(0, 201, 255, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
}

.hero-content {
  max-width: 800px;
  animation: fadeIn 1s ease-in-out;
}

.hero-title {
  font-size: 3.5em;
  color: var(--primary-color);
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 3px;
  margin-bottom: 20px;
  animation: slideInDown 1s ease-out;
}

.hero-subtitle {
  color: #d1d1d1;
  font-size: 1.5em;
  line-height: 1.5;
  animation: slideInUp 1s ease-out;
}

.content-card {
  background: rgba(255, 255, 255, 0.07);
  backdrop-filter: blur(15px);
  border-radius: 20px;
  padding: 4rem;
  box-shadow: 0 16px 32px rgba(0, 201, 255, 0.15);
  text-align: left;
  transition: all var(--transition-speed) ease;
  
  /* Dynamic width */
  /* width: 90%; */
  max-width: 1100px;
  /* margin: 40px auto; */
}

.search-bar {
  display: flex;
  align-items: center;
  margin-bottom: 50px;
  position: relative;
}

.search-input {
  flex: 1;
  background-color: rgba(255, 255, 255, 0.1);
  border: 2px solid transparent;
  padding: 15px 20px;
  font-size: 16px;
  color: var(--secondary-color);
  border-radius: 30px;
  transition: background-color var(--transition-speed), border-color var(--transition-speed);
}

.search-input:focus {
  background-color: rgba(255, 255, 255, 0.2);
  border-color: var(--primary-color);
  outline: none;
}

.search-button {
  background-color: var(--primary-color);
  color: var(--background-color);
  border: none;
  border-radius: 50%;
  padding: 12px;
  margin-left: -50px;
  cursor: pointer;
  transition: background-color var(--transition-speed), transform var(--transition-speed);
  display: flex;
  align-items: center;
  justify-content: center;
}

.search-button:hover {
  background-color: var(--accent-color);
  transform: scale(1.05);
}

.search-icon {
  width: 20px;
  height: 20px;
}

.section-title {
  font-size: 2.5em;
  color: var(--primary-color);
  margin-bottom: 30px;
  text-transform: uppercase;
  letter-spacing: 2px;
  border-bottom: 2px solid var(--primary-color);
  padding-bottom: 10px;
  animation: fadeInUp 1s ease-in-out;
}

.featured-section,
.products-section {
  margin-top: 60px;
}

@media (max-width: 1024px) {
  .hero-section {
    padding: 80px 20px;
  }
  .hero-title {
    font-size: 3em;
  }
  .hero-subtitle {
    font-size: 1.3em;
  }
}

@media (max-width: 768px) {
  .hero-section {
    padding: 60px 15px;
  }
  .hero-title {
    font-size: 2.5em;
  }
  .hero-subtitle {
    font-size: 1.1em;
  }
  .search-bar {
    flex-direction: column;
    align-items: stretch;
  }
  .search-button {
    margin-left: 0;
    margin-top: 15px;
    align-self: flex-end;
  }
}

@media (max-width: 480px) {
  .hero-section {
    padding: 40px 10px;
  }
  .hero-title {
    font-size: 2em;
  }
  .hero-subtitle {
    font-size: 1em;
  }
  .section-title {
    font-size: 2em;
  }
}

/* Animations */
@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes slideInDown {
  from {
    transform: translateY(-30px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

@keyframes slideInUp {
  from {
    transform: translateY(30px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
