<template>
	<div class="about">
		<div class="profile-card" @mouseover="rotateCard" @mouseleave="resetCard">
			<!-- <img src="/path-to-your-profile.jpg" alt="Profile Picture" class="profile-picture" /> -->
			<h1>Skuld Norniern</h1>
			<div class="social-links">
				<a href="https://github.com/SkuldNorniern" target="_blank">GitHub</a>
			</div>
			<div class="about-me">
				<h2>About Me</h2>
				<p>Welcome to my blog! Here you can find information about by dev log, and my story.</p>
				<p>Avalible Stacks: C/C++, Rust, Go, Python</p>
			</div>
		</div>
	</div>
</template>

<script lang="ts">
export default {
  methods: {
    rotateCard(event) {
      const card = event.currentTarget;
      const { width, height, left, top } = card.getBoundingClientRect();
      const l = event.clientX - (left + window.scrollX);
      const t = event.clientY - (top + window.scrollY);
      const centerX = width / 2;
      const centerY = height / 2;
      const rotateX = (centerY - t) / 20; // Reduced for subtlety
      const rotateY = (l - centerX) / 20; // Reduced for subtlety

      card.style.transform = `perspective(1000px) rotateX(${rotateX}deg) rotateY(${rotateY}deg)`;
    },
    resetCard(event) {
      event.currentTarget.style.transform = 'perspective(1000px) rotateX(0) rotateY(0)';
    }
  }
};
</script>

<style scoped>
:root {
  --primary-color: #00ffff;
  --secondary-color: #ffffff;
  --background-color: #0a0a0a;
  --font-family: 'Roboto', sans-serif;
}

.about {
  font-family: var(--font-family);
  color: var(--secondary-color);
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
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  padding-top: 2rem;
}

/* Remove the following styles as they're no longer needed */
/* .profile-picture { ... } */
/* .about-me h2 { ... } */
/* .about-me p { ... } */

</style>
