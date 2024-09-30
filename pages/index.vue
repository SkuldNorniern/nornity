<template>
  <div class="container">
    <!-- Navigation -->
    <nav class="navigation">
      <div class="search-container">
        <input
          type="text"
          placeholder="Search articles..."
          class="search-input"
          v-model="searchQuery"
          @input="performSearch"
        />
        <!-- Floating Search Results Panel -->
        <div class="floating-search-results" v-if="filteredArticles.length">
          <SearchResults
            :filteredArticles="filteredArticles"
            :searchQuery="searchQuery"
          />
        </div>
      </div>
    </nav>

    <!-- Main Content -->
    <main class="main-content">
      <div class="left-column">
        <!-- Featured Articles -->
        <section
          class="featured-article"
          v-for="article in latestArticle"
          :key="article.slug"
        >
          <div class="floating-orb"></div>
          <div class="article-content">
            <div class="article-meta">Latest Article</div>
            <h1 class="article-title">{{ article.title }}</h1>
            <p>{{ article.date }}</p>
            <p>{{ truncateDescription(article.description, 100) }}</p>
            <a :href="`/articles/${article.slug}`" class="read-more">
              Read article
            </a>
          </div>
        </section>

        <!-- About Section -->
        <section class="featured-article about-section">
          <div class="article-content">
            <div class="about-header">
              <img
                src="https://avatars.githubusercontent.com/u/43695854?v=4"
                alt="Profile Picture"
                class="profile-picture"
              />
              <h1 class="article-title">Skuld Norniern</h1>
            </div>

            <div class="about-me">
              <h2>About Me</h2>
              <p>
                Welcome to my blog! Here you can find information about my dev log,
                and my story.
              </p>
              <p>Available Stacks: C/C++, Rust, Go, Python</p>
            </div>

            <div class="social-links">
              <a href="https://github.com/SkuldNorniern" target="_blank">
                GitHub
              </a>
              <a href="https://twitter.com/SkuldNorniern" target="_blank">
                Twitter
              </a>
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

        <!-- Enhanced Support Section -->
        <section class="support-section">
          <h2 class="section-title">Stay Connected & Support</h2>
          <div class="support-content">
            <!-- RSS Feed Link with Copy Button -->
            <div class="rss-feed">
              <i class="fas fa-rss icon-rss"></i>
              <a href="/rss.xml" target="_blank" rel="noopener noreferrer">
                Subscribe to RSS Feed
              </a>
              <button
                @click="copyRSSLink"
                class="copy-button"
                :class="{ success: copySuccess, error: copyError }"
              >
                <i class="fas fa-copy"></i>
                {{ copySuccess ? 'Copied!' : 'Copy Link' }}
              </button>
            </div>

            <!-- Buy Me a Coffee Button -->
            <div class="buy-me-coffee">
              <a
                href="https://www.buymeacoffee.com/SkuldNorniern"
                target="_blank"
                rel="noopener noreferrer"
                class="coffee-button"
              >
                <i class="fas fa-coffee"></i> Buy Me a Coffee
              </a>
            </div>
          </div>

          <div class="support-background">
            <div class="bg-shape shape-1"></div>
            <div class="bg-shape shape-2"></div>
            <div class="bg-shape shape-3"></div>
          </div>
        </section>
      </div>

      <div class="right-column">
        <!-- Recommended Articles -->
        <section class="recommended-articles">
          <h2 class="section-title">Recommended</h2>
          <div class="article-grid" ref="articleGrid">
            <article
              class="article-card"
              v-for="article in displayedArticles"
              :key="article.slug"
            >
              <div class="article-content">
                <h3>{{ article.title }}</h3>
                <p class="article-date">{{ article.date }}</p>
                <p class="article-description">
                  {{ truncateDescription(article.description, 100) }}
                </p>
              </div>
              <a :href="`/articles/${article.slug}`" class="read-more-light">
                Read article
              </a>
            </article>
          </div>
          <button
            v-if="displayedArticles.length < articles.length"
            @click="loadMoreArticles"
            class="load-more-button"
          >
            Load More
          </button>
        </section>
      </div>
    </main>
  </div>
</template>

<script lang="ts">
import {
  defineComponent,
  ref,
  watch,
  onMounted,
  computed,
} from 'vue';
import FeaturedArticles from '@/components/FeaturedArticles.vue';
import FeaturedProducts from '@/components/FeaturedProducts.vue';
import SearchResults from '@/components/SearchResults.vue';

interface Article {
  title: string;
  slug: string;
  date: string;
  description: string;
  [key: string]: any;
}

export default defineComponent({
  components: {
    FeaturedArticles,
    FeaturedProducts,
    SearchResults,
  },
  setup() {
    const searchQuery = ref('');
    const articles = ref<Article[]>([]);
    const latestArticle = ref<Article[]>([]);
    const filteredArticles = ref<Article[]>([]);
    const featuredArticles = ref<Article[]>([]);
    const featuredProducts = ref<any[]>([]);
    const isLoading = ref(false);
    const articleGrid = ref<HTMLElement | null>(null);
    const displayedArticles = ref<Article[]>([]);
    const articlesPerPage = ref(6);

    const copySuccess = ref(false);
    const copyError = ref(false);

    const fetchContent = async (
      contentType: string,
      targetRef: typeof articles | typeof featuredArticles | typeof featuredProducts
    ) => {
      try {
        isLoading.value = true;
        const fetchedData = await queryContent(contentType)
          .where({ _extension: 'md' })
          .find();
        targetRef.value = fetchedData;
        console.log(`${contentType} List:`, targetRef.value);
      } catch (error) {
        console.error(`Error fetching ${contentType}:`, error);
      } finally {
        isLoading.value = false;
      }
    };

    const fetchLatestArticle = async () => {
      try {
        isLoading.value = true;
        const fetchedData = await queryContent('articles')
          .where({ _extension: 'md' })
          .sort({ date: -1 })
          .limit(1)
          .find();
        latestArticle.value = fetchedData;
        console.log('Latest Article:', latestArticle.value);
      } catch (error) {
        console.error('Error fetching latest article:', error);
      } finally {
        isLoading.value = false;
      }
    };

    const searchArticles = () => {
      const query = searchQuery.value.trim().toLowerCase();
      filteredArticles.value = query
        ? articles.value.filter((article) =>
            article.title.toLowerCase().includes(query)
          )
        : [];
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
      console.log('Displayed articles:', displayedArticles.value);
    };

    const loadMoreArticles = () => {
      const currentLength = displayedArticles.value.length;
      const newArticles = articles.value.slice(
        currentLength,
        currentLength + articlesPerPage.value
      );
      displayedArticles.value = [...displayedArticles.value, ...newArticles];
    };

    const copyRSSLink = async () => {
      const rssLink = `${window.location.origin}/rss.xml`;

      if (process.client) {
        try {
          if (navigator.clipboard && navigator.clipboard.writeText) {
            await navigator.clipboard.writeText(rssLink);
          } else {
            // Fallback for older browsers
            const textArea = document.createElement('textarea');
            textArea.value = rssLink;
            document.body.appendChild(textArea);
            textArea.select();
            document.execCommand('copy');
            document.body.removeChild(textArea);
          }
          copySuccess.value = true;
          copyError.value = false;
        } catch (error) {
          console.error('Failed to copy RSS link:', error);
          copyError.value = true;
          copySuccess.value = false;
        } finally {
          setTimeout(() => {
            copySuccess.value = false;
            copyError.value = false;
          }, 2000);
        }
      } else {
        console.warn('Copying is not available during server-side rendering');
      }
    };

    const truncateDescription = (description: string, maxLength: number) => {
      return description.length <= maxLength
        ? description
        : `${description.slice(0, maxLength)}...`;
    };

    watch(searchQuery, searchArticles, { immediate: true });

    onMounted(async () => {
      await Promise.all([
        fetchContent('articles', articles),
        fetchLatestArticle(),
        fetchContent('featured-articles', featuredArticles),
        fetchContent('featured-products', featuredProducts),
      ]);

      updateDisplayedArticles();
      window.addEventListener('resize', updateDisplayedArticles);
    });

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
      copySuccess,
      copyError,
      copyRSSLink,
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
  max-width: 800px;
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
  max-height: 50vh;
  position: relative;
}

.left-column {
  display: flex;
  flex-direction: column;
  gap: 30px;
}

.right-column {
  position: relative;
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
  0%,
  100% {
    transform: translate(0, 0);
  }
  25% {
    transform: translate(50%, 25%);
  }
  50% {
    transform: translate(0, 50%);
  }
  75% {
    transform: translate(-50%, 25%);
  }
}

.recommended-articles {
  background: linear-gradient(135deg, #ffffff, #cccccc);
  color: black;
  border-radius: 20px;
  padding: 30px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  min-height: 400px;
}

.article-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  grid-template-rows: repeat(auto-fill, minmax(220px, 1fr));
  gap: 20px;
  row-gap: 60px;
  padding: 10px;
  overflow-y: auto;
  padding-right: 15px;
  min-height: 300px;
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

/* Responsive Styles */
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

  .support-section {
    padding: 30px 20px;
  }

  .rss-feed,
  .buy-me-coffee {
    flex-direction: column;
  }

  .copy-button,
  .coffee-button {
    width: 100%;
    justify-content: center;
  }
}

.floating-search-results {
  position: absolute;
  margin-top: 50px;
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

/* Enhanced Support Section Styles */
.support-section {
  background: linear-gradient(135deg, #f0f0f0, #e0e0e0);
  padding: 40px;
  border-radius: 20px;
  text-align: center;
  position: relative;
  overflow: hidden;
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.1);
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.support-section:hover {
  transform: translateY(-5px);
  box-shadow: 0 15px 30px rgba(0, 0, 0, 0.15);
}

.section-title {
  font-size: 28px;
  margin-bottom: 30px;
  color: var(--primary-color);
  position: relative;
  display: inline-block;
}

.section-title::after {
  content: '';
  position: absolute;
  bottom: -10px;
  left: 50%;
  transform: translateX(-50%);
  width: 50px;
  height: 3px;
  background-color: var(--accent-color);
}

.support-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 30px;
  position: relative;
  z-index: 1;
}

.rss-feed,
.buy-me-coffee {
  display: flex;
  align-items: center;
  gap: 15px;
}

.icon-rss {
  font-size: 24px;
  color: #ffa500;
}

.rss-feed a {
  color: var(--primary-color);
  text-decoration: none;
  font-size: 18px;
  font-weight: 500;
  transition: color 0.3s ease;
}

.rss-feed a:hover {
  color: var(--accent-color);
}

.copy-button,
.coffee-button {
  padding: 10px 20px;
  border: none;
  border-radius: 30px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 8px;
}

.copy-button {
  background-color: var(--accent-color);
  color: white;
}

.copy-button:hover {
  background-color: #e91e63;
  transform: translateY(-2px);
}

.copy-button.success {
  background-color: #4caf50;
}

.copy-button.error {
  background-color: #f44336;
}

.coffee-button {
  background-color: #ff813f;
  color: white;
  text-decoration: none;
}

.coffee-button:hover {
  background-color: #e67334;
  transform: translateY(-2px);
}

.support-background {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  overflow: hidden;
}

.bg-shape {
  position: absolute;
  border-radius: 50%;
  opacity: 0.1;
}

.shape-1 {
  width: 150px;
  height: 150px;
  background-color: var(--primary-color);
  top: -50px;
  left: -50px;
}

.shape-2 {
  width: 100px;
  height: 100px;
  background-color: var(--accent-color);
  bottom: -30px;
  right: -30px;
}

.shape-3 {
  width: 80px;
  height: 80px;
  background-color: #ffa500;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}
</style>