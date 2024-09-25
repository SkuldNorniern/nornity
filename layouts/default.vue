<template>
  <div id="app">
    <main>
      <slot />
    </main>

    <!-- <footer class="navbar">
      <div class="container footer-content">
        <nav class="footer-nav">
          <a class="navbar-item" href="/">Home</a>
          <a class="navbar-item" href="/about">About</a>
        </nav>  
        <div class="footer-info">
          <p>© 2024 SkuldNorniern. All rights reserved.</p>
          <p>Follow on <a href="https://github.com/SkuldNorniern" target="_blank">GitHub</a>.</p>
        </div>
      </div>
    </footer> -->

    <!-- Dark mode toggle as a floating icon -->
    <div class="dark-mode-toggle">
      <button class="toggle-button" @click="toggleDarkMode">
        <i :class="isDarkMode ? 'fas fa-sun' : 'fas fa-moon'"></i>
      </button>
    </div>

    <!-- Go Back floating button -->
    <div class="go-back-button">
      <button class="back-button" @click="goBack">
        <i class="fas fa-arrow-left"></i>
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { ref, onMounted } from 'vue';

export default {
  setup() {
    const isDarkMode = ref(false);

    onMounted(() => {
      const darkModeSetting = localStorage.getItem('darkMode');
      isDarkMode.value = darkModeSetting === 'true';
      document.body.classList.toggle('dark-mode', isDarkMode.value);
    });

    const toggleDarkMode = () => {
      isDarkMode.value = !isDarkMode.value;
      document.body.classList.toggle('dark-mode', isDarkMode.value);
      localStorage.setItem('darkMode', isDarkMode.value.toString());
    };

    const goBack = () => {
      window.history.back();
    };

    return { isDarkMode, toggleDarkMode, goBack };
  },
};
</script>

<style scoped>
@import url('https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.4/css/all.min.css');

* {
  /* Reset some default styles */
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  width: 100%;
  height: 100%;
}

#app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  transition: background-color 0.3s ease, color 0.3s ease;
}

main {
  flex-grow: 1;
  width: 100%;
  transition: background-color 0.3s ease, color 0.3s ease;
}

.navbar {
  display: flex;
  width: 100%;
  background-color: #24292e;
  color: white;
  padding: 1rem;
  position: fixed;
  bottom: 0;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 -4px 10px rgba(0, 0, 0, 0.2);
}

.footer-content {
  display: flex;
  width: 100%;
  justify-content: space-between;
  align-items: center;
}

.footer-nav {
  display: flex;
  list-style: none;
}

.navbar-item {
  color: white;
  text-decoration: none;
  padding: 0 1rem;
  font-weight: 500;
  transition: color 0.3s;
}

.navbar-item:hover {
  color: #00ffff;
}

.toggle-button {
  background-color: #4B0082; /* Dark violet for light mode */
  border: none;
  color: white;
  padding: 0.5rem;
  cursor: pointer;
  font-size: 1.2rem;
  transition: background-color 0.3s ease, color 0.3s ease, opacity 0.3s ease;
  border-radius: 50%;
  position: fixed;
  bottom: 2rem;
  right: 2rem;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
}

.toggle-button:hover {
  opacity: 0.8; /* Add a hover effect */
}

.footer-info {
  text-align: center;
  padding-top: 1rem;
  color: #9da5b4;
  font-size: 0.9rem;
}

.footer-info a {
  color: #9da5b4;
  text-decoration: none;
  transition: color 0.3s ease;
}

.footer-info a:hover {
  color: white;
}

body.dark-mode {
  background-color: #1d1d1d; 
  color: #c9d1d9;
  transition: background-color 1s ease, color 1s ease;
}

body.dark-mode .navbar {
  background-color: #2c2c2e;
}

body.dark-mode .navbar-item,
body.dark-mode .toggle-button,
body.dark-mode .footer-info p,
body.dark-mode .footer-info a {
  color: #c9d1d9;
}

body.dark-mode .toggle-button {
  background-color: #ADD8E6; /* Light blue for dark mode */
  color: #1d1d1d; /* Dark text color for better contrast */
}

body.dark-mode .toggle-button {
  border-color: #c9d1d9;
}

body.dark-mode .toggle-button:hover {
  opacity: 0.8; /* Add a hover effect */
}

@media (max-width: 768px) {
  .toggle-button {
    padding: 0.4rem;
    font-size: 1rem;
  }

  .navbar {
    flex-direction: column;
    align-items: center; /* Center align items */
    padding: 1rem;
  }

  .footer-content {
    flex-direction: column;
    align-items: center; /* Center align items */
    text-align: center; /* Center text */
  }

  .footer-nav {
    margin-bottom: 1rem;
    gap: 1rem;
    justify-content: center; /* Center navigation items */
  }

  .footer-info {
    padding-top: 0.5rem; /* Reduce padding for better spacing */
  }
}

/* Add this block near the top of your style section */
body, body *, .navbar, .navbar-item, .toggle-button, .footer-info p, .footer-info a {
  transition: background-color 0.3s ease, color 0.3s ease;
}

.go-back-button {
  position: fixed;
  bottom: 2rem;
  left: 2rem;
  z-index: 1000;
}

.back-button {
  background-color: #4B0082; /* Dark violet for light mode */
  border: none;
  color: white;
  padding: 0.5rem;
  cursor: pointer;
  font-size: 1.2rem;
  transition: background-color 0.3s ease, color 0.3s ease, opacity 0.3s ease;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.back-button:hover {
  opacity: 0.8; /* Add a hover effect */
}

body.dark-mode .back-button {
  background-color: #ADD8E6; /* Light blue for dark mode */
  color: #1d1d1d; /* Dark text color for better contrast */
}

@media (max-width: 768px) {
  .back-button {
    padding: 0.4rem;
    font-size: 1rem;
  }
}


</style>
