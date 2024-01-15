<template>
	<div id="app">
		<header class="blog-header">
			<div class="container">
				<h1>Nornity</h1>
			</div>
		</header>
		<main>
			<AppHeader />
			<slot />
			<AppFooter />a
		</main>

		<footer class="navbar">
			<div class="container footer-content">
				<nav class="footer-nav">
					<a class="navbar-item" href="/">Home</a>
					<a class="navbar-item" href="/about">About</a>
				</nav>
				<!-- Dark Mode Toggle Button -->
				<button class="toggle-button" @click="toggleDarkMode">Toggle Dark Mode</button>
				<div class="footer-info">
					<p>© 2024 SkuldNorniern. All rights reserved.</p>
					<p>Follow on <a href="https://github.com/SkuldNorniern">GitHub</a>.</p>
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
#app {
	display: flex;
	flex-direction: column;
	min-height: 100vh;
}

.blog-header {
	background-color: #f8f9fa;
	padding: 1rem 0;
	text-align: center;
	border-bottom: 1px solid #e1e4e8;
}

.blog-header h1 {
	color: #0366d6;
	margin: 0;
	font-size: 2rem;
}

.main {
	flex: 1;
	padding-bottom: 100px; /* Adjust the value to match your footer height */
}


.navbar {
	background-color: #24292e;
	color: white;
	padding: 1rem;
	position: fixed;
	bottom: 0;
	width: 100%;
	display: flex;
	justify-content: space-between;
	align-items: center;
}

.footer-content {
	display: flex;
	justify-content: space-between;
	align-items: center;
	flex-wrap: wrap;
}

.footer-nav {
	display: flex;
	list-style: none;
}

.navbar-item {
	color: white;
	text-decoration: none;
	padding: 0 1rem;
}

.toggle-button {
	background: none;
	border: 1px solid white;
	color: white;
	padding: 0.5rem 1rem;
	cursor: pointer;
}

.footer-info {
	text-align: right;
}

.footer-info p {
	color: #9da5b4;
	margin: 0;
}

.footer-info a {
	color: #9da5b4;
	text-decoration: none;
	transition: color 0.3s ease;
}

.footer-info a:hover {
	color: white;
}

/* Dark mode styles */
body.dark-mode {
	background-color: #0d1117;
	color: #c9d1d9;
}

body.dark-mode .blog-header {
	background-color: #161b22;
	border-color: #303
		63d;
}

body.dark-mode .navbar {
	background-color: #161b22;
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
</style>
