<template>
  <div id="app">
    <main>
      <slot />
    </main>

    <footer class="navbar">
      <div class="container footer-content">
        <nav class="footer-nav">
          <a class="navbar-item" href="/">Home</a>
          <a class="navbar-item" href="/about">About</a>
        </nav>
        <button class="toggle-button" @click="toggleDarkMode">
          {{ isDarkMode ? 'Light Mode' : 'Dark Mode' }}
        </button>
        <div class="footer-info">
          <p>© 2024 SkuldNorniern. All rights reserved.</p>
          <p>Follow on <a href="https://github.com/SkuldNorniern" target="_blank">GitHub</a>.</p>
        </div>
      </div>
    </footer>
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

    return { isDarkMode, toggleDarkMode };
  },
};
</script>

<style scoped>
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
}

main {
  flex-grow: 1;
  width: 100%;
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
  background: none;
  border: 2px solid white;
  color: white;
  padding: 0.5rem 1rem;
  cursor: pointer;
  font-size: 0.9rem;
  letter-spacing: 0.5px;
  transition: background-color 0.3s ease, border-color 0.3s ease;
  border-radius: 15px;
}

.toggle-button:hover {
  background-color: #0366d6;
  border-color: #0366d6;
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
  border-color: #c9d1d9;
}

body.dark-mode .toggle-button:hover {
  background-color: #58a6ff;
  border-color: #58a6ff;
}

@media (max-width: 768px) {
  .toggle-button {
    padding: 0.4rem 0.8rem;
    font-size: 0.8rem;
  }

  .navbar {
    flex-direction: column;
    align-items: flex-start;
  }

  .footer-content {
    flex-direction: column;
    align-items: flex-start;
  }

  .footer-nav {
    margin-bottom: 1rem;
  }
}
</style>
