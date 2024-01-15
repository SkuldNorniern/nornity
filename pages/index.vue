<template>
  <div class="container">
    <section class="hero">
      <h1>Welcome to My Modern Blog</h1>
      <p>Explore the latest articles, insights, and trends in our blog.</p>
    </section>

    <section class="featured">
      <h2>Featured Articles</h2>
      <div class="featured-grid">
        <!-- Dynamically load featured articles -->
        <div class="featured-item" v-for="article in featuredArticles" :key="article.id">
          <nuxt-link :to="`/articles/${article.slug}`">
            <img :src="article.image" alt="" class="featured-image">
            <div class="article-details">
              <h3>{{ article.title }}</h3>
            </div>
          </nuxt-link>
        </div>
      </div>
    </section>

    <section class="products">
      <h2>Our Products</h2>
      <div class="products-grid">
        <!-- Dynamically load products -->
        <div class="product-item" v-for="product in products" :key="product.id">
          <nuxt-link :to="`/products/${product.slug}`">
            <img :src="product.image" alt="">
            <div class="product-details">
              <h3>{{ product.name }}</h3>
            </div>
          </nuxt-link>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
export default {
  data() {
    return {
      featuredArticles: [
        // ...existing articles
      ],
      products: [
        // ...existing products
      ],
    };
  },
};
</script>

<style scoped>
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.hero {
  text-align: center;
  padding: 50px 20px;
  background-color: #f6f8fa;
  border: 1px solid #e1e4e8;
  border-radius: 6px;
  margin-top: 20px;
}

.hero h1 {
  font-size: 2.5em;
  color: #0366d6;
}

.hero p {
  color: #586069;
  font-size: 1.1em;
}

.featured, .products {
  margin-top: 40px;
}

.featured-grid, .products-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 20px;
}

.featured-item, .product-item {
  text-align: center;
  border: 1px solid #e1e4e8;
  border-radius: 6px;
  overflow: hidden;
  transition: transform 0.3s ease;
}

.featured-item:hover, .product-item:hover {
  transform: translateY(-5px);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.featured-image {
  width: 100%;
  height: 200px;
  object-fit: cover;
}

.article-details, .product-details {
  padding: 15px;
}

/* Dark mode styles */
body.dark-mode {
  background-color: #121212;
  color: #c9d1d9;
}

body.dark-mode .hero {
  background-color: #1c1c1e;
  border-color: #343436;
}

body.dark-mode .featured-item, 
body.dark-mode .product-item {
  background-color: #2c2c2e;
  border-color: #4a4
a4c;
}

body.dark-mode .featured-item h3,
body.dark-mode .product-item h3 {
color: #fff;
}

body.dark-mode .navbar {
background-color: #181818;
border-color: #333;
}

body.dark-mode .navbar-item,
body.dark-mode .footer-info p,
body.dark-mode .footer-info a {
color: #c9d1d9;
}

body.dark-mode .footer-info a:hover {
color: #fff;
}

/* Hover effects for light mode */
.featured-item:hover .article-details h3,
.product-item:hover .product-details h3 {
color: #0366d6;
}

/* Additional global dark mode styles */
body.dark-mode {
--bg-color: #0d1117;
--text-primary: #c9d1d9;
--text-secondary: #8b949e;
--border-color: #30363d;
--header-bg-color: #161b22;

--link-color: #58a6ff;
}

body.dark-mode {
  background-color: #121212; /* Dark background for dark mode */
  color: #c9d1d9;
}

body.dark-mode .hero {
  background-color: #1c1c1e;
  border-color: #343436;
}

body.dark-mode .featured-item, 
body.dark-mode .product-item {
  background-color: #2c2c2e;
  border-color: #4a4a4c;
}

body.dark-mode .featured-item h3,
body.dark-mode .product-item h3 {
  color: #fff;
}

body.dark-mode .navbar {
  background-color: #181818;
  border-color: #333;
}

body.dark-mode .navbar-item,
body.dark-mode .footer-info p,
body.dark-mode .footer-info a {
  color: #c9d1d9;
}

body.dark-mode .footer-info a:hover {
  color: #fff;
}

body.dark-mode .container {
  background-color: #121212;
}

/* If the application is not scoped, remove scoped attribute and add the global selector */
html.dark-mode {
  background-color: #121212; /* Set the background color for the entire page in dark mode */
}

/* Additional global styles for dark mode */
body.dark-mode {
  --bg-color: #0d1117; /* You can define CSS variables for easy theme management */
  --text-primary: #c9d1d9;
  --text-secondary: #8b949e;
  --border-color: #30363d;
  --header-bg-color: #161b22;
  --link-color: #58a6ff;
}

/* Apply dark mode variables */
body.dark-mode {
  background-color: var(--bg-color);
  color: var(--text-primary);
}

/* Footer styles for dark mode */
body.dark-mode footer.navbar {
  background-color: var(--header-bg-color);
  border-top: 1px solid var(--border-color);
}

/* Ensure footer content is legible in dark mode */
body.dark-mode .footer-content {
  color: var(--text-secondary);
}
body {
  background-color: #ffffff; /* Default light background */
  color: #333333; /* Default text color for light mode */
}

/* Ensure that all text elements have a color set for dark mode */
body.dark-mode,
body.dark-mode h1,
body.dark-mode h2,
body.dark-mode h3,
body.dark-mode p,
body.dark-mode a {
  color: #c9d1d9; /* Light text color for dark mode */
}

/* Dark mode specific styles */
body.dark-mode {
  background-color: #121212; /* Dark background for dark mode */
}

body.dark-mode .navbar,
body.dark-mode .footer-content,
body.dark-mode .footer-info a {
  background-color: #181818; /* Slightly lighter background for navbar and footer */
  color: #c9d1d9; /* Text color for navbar and footer links */
}

body.dark-mode .footer-info a:hover {
  color: #ffffff; /* Hover color for links */
}

body.dark-mode .hero {
  background-color: #1c1c1e;
  border-color: #343436;
}

body.dark-mode .featured-item,
body.dark-mode .product-item {
  background-color: #2c2c2e;
  border-color: #4a4a4c;
}
/* Adjust navbar links for dark mode */
body.dark-mode .navbar-item {
  color: var(--text-primary);
}

body.dark-mode .navbar-item:hover {
  color: var(--link-color);
}

/* Button styling for dark mode */
body.dark-mode .toggle-button {
  background-color: var(--header-bg-color);
  color: var(--text-primary);
  border-color: var(--border-color);
}

body.dark-mode .toggle-button:hover {
  background-color: var(--border-color);
}
</style>
