<template>
  <div class="product-container">
    <h1>{{ product.name }}</h1>
    <img :src="product.image" alt="">
    <p>{{ product.description }}</p>
    <!-- Add more product details -->
  </div>
</template>

<script>
export default {
  data() {
    return {
      // Replace with real product data
      product: {
        name: 'Product Name',
        image: 'path-to-product-image.jpg',
        description: 'Product description goes here...'
      },
    };
  },
};
</script>

<style scoped>
/* Product page styles */
.product-container {
  max-width: 800px;
  margin: 20px auto;
  padding: 20px;
}
</style>

