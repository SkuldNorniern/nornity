<template>
  <div class="comment-section">
    <h3>Comments</h3>
    <div class="comments">
      <div class="comment" v-for="comment in comments" :key="comment.date">
        <p class="comment-author">{{ comment.author }}</p>
        <p class="comment-date">{{ comment.date }}</p>
        <p class="comment-content">{{ comment.comment }}</p>
      </div>
    </div>
    <div class="comment-form">
      <textarea v-model="newComment" placeholder="Add a comment..."></textarea>
      <button @click="postComment">Post Comment</button>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  props: {
    comments: Array,
  },
  data() {
    return {
      newComment: '',
    };
  },
  methods: {
    postComment() {
      // Placeholder function to handle comment posting
      alert(`Posting comment: ${this.newComment}`);
      this.newComment = '';
    },
  },
};
</script>

<style scoped>
.comment-section {
  margin-top: 40px;
}

.comments {
  margin-bottom: 20px;
}

.comment {
  margin-bottom: 10px;
  padding-bottom: 10px;
  border-bottom: 1px solid #e1e4e8;
}

.comment-author {
  font-weight: bold;
  color: #0366d6;
}

.comment-date {
  font-size: 0.9em;
  color: #586069;
}

.comment-content {
  color: #24292e;
}

.comment-form textarea {
  width: 100%;
  margin-bottom: 10px;
}

/* Dark mode styles */
body.dark-mode .comment {
  color: #cccccc;
  border-color: #444444;
}
</style>

