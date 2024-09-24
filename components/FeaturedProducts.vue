<template>
  <section class="featured-products">
    <h2>Our Products</h2>
    <div class="products-grid">
      <div class="product-card" v-for="product in products" :key="product.slug">
        <nuxt-link :to="`/products/${product.slug}`">
          <div class="card-image">
            <img :src="product.image" alt="{{ product.name }}" />
          </div>
          <div class="card-content">
            <h3>{{ product.name }}</h3>
          </div>
        </nuxt-link>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
defineProps<{
  products: Array<{
    slug: string;
    image: string;
    name: string;
  }>;
}>();
</script>

<style scoped>
.featured-products {
  padding: 60px 0;
}

.featured-products h2 {
  font-size: 2rem;
  font-weight: 600;
  text-align: center;
  color: var(--color-text);
  margin-bottom: 40px;
}

.products-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  justify-content: center;
}

.product-card {
  background-color: #fff;
  border-radius: 12px;
  overflow: hidden;
  width: 300px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  transition: transform 0.3s;
}

.product-card:hover {
  transform: translateY(-5px);
}

.card-image img {
  width: 100%;
  height: auto;
  display: block;
}

.card-content {
  padding: 20px;
}

.card-content h3 {
  font-size: 1.25rem;
  font-weight: 500;
  color: var(--color-text);
  margin: 0;
}

@media (max-width: 768px) {
  .product-card {
    width: 100%;
  }
}
</style>

