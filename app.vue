<template>
  <div>
    <NuxtLayout>
      <NuxtPage/>
    </NuxtLayout>
  </div>
</template>

<script lang="ts">
import { ref, onMounted } from 'vue';

export default {
  setup() {
    const isDarkMode = ref(false);

    onMounted(() => {
      isDarkMode.value = localStorage.getItem('darkMode') === 'true';
      if (isDarkMode.value) {
        document.body.classList.add('dark-mode');
      }
    });

    const toggleDarkMode = () => {
      isDarkMode.value = !isDarkMode.value;
      document.body.classList.toggle('dark-mode', isDarkMode.value);
      localStorage.setItem('darkMode', isDarkMode.value ? 'true' : 'false');
    };

    return { isDarkMode, toggleDarkMode };
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

.features {
	display: flex;
	justify-content: space-around;
	margin-top: 40px;
}

.feature-card {
	flex-basis: 30%;
	background-color: white;
	padding: 20px;
	border: 1px solid #e1e4e8;
	border-radius: 6px;
	box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	text-align: center;
}

.feature-card h3 {
	font-size: 1.2em;
	color: #24292e;
	margin-bottom: 10px;
}

.feature-card p {
	font-size: 1em;
	color: #333333;
}
body.dark-mode {
	background-color: #1e1e1e;
	color: #cccccc;
}

body.dark-mode header {
	background-color: #333333;
	color: #ffffff;
}

body.dark-mode .navbar {
	border-bottom: 1px solid #444444;
}

body.dark-mode .hero {
	background-color: #2a2a2a;
	border-color: #3a3a3a;
}

body.dark-mode .feature-card {
	background-color: #333333;
	border-color: #444444;
	color: #ffffff;
}

</style>

