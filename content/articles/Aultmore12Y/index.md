---
title: "Aultmore 12Y Tasting Notes"
slug: aultmore-12y
image: "./images/aultmore-12y.jpg"
language: "English"
tags: ["Whisky", "Scotch", "Single Malt"]
date: "2024-10-13"
author: "Skuld Norniern"
---

<style>
  @import url('https://fonts.googleapis.com/css2?family=Cormorant+Garamond:wght@400;600&family=Montserrat:wght@300;400&display=swap');

  body {
    background-color: #0a0a0a;
    background-image: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%23ffffff' fill-opacity='0.03'%3E%3Cpath d='M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
  }
  .glass-article {
    max-width: 900px;
    margin: 20px auto;
    padding: 20px;
    font-family: 'Lato', sans-serif;
    color: #e0e0e0;
    line-height: 1.8;
    background-color: rgba(20, 20, 20, 0.95);
    border-radius: 12px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  .article-header {
    text-align: center;
    margin-bottom: 40px;
  }
  .title {
    font-family: 'Playfair Display', serif;
    font-size: 3em;
    color: #d4af37;
    margin-bottom: 20px;
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
  }
  .article-image {
    max-width: 70%;
    /* transform: rotate(90deg); */
    margin: 60px auto;
    height: auto;
    border-radius: 12px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.6);
    border: 2px solid rgba(212, 175, 55, 0.3); /* Golden border */
    transition: transform 0.3s ease, box-shadow 0.3s ease;
    margin-bottom: 30px;
  }

  .tasting-notes {
    background-color: rgba(40, 40, 40, 0.7);
    margin: 0 auto;
    padding: 30px;
    border-radius: 8px;
    margin-top: 40px;
    border: 1px solid rgba(255, 255, 255, 0.05);
  }
  .section-title {
    font-family: 'Playfair Display', serif;
    color: #d4af37;
    border-bottom: 2px solid #d4af37;
    padding-bottom: 10px;
    margin-bottom: 30px;
  }
  .section-intro {
    font-family: 'Playfair Display', serif;
    color: #d4af37;
    margin-bottom: 30px;
    border-bottom: 2px solid #d4af37;
  }
  .note-category {
    margin-bottom: 30px;
  }
  .note-category h3 {
    font-family: 'Playfair Display', serif;
    color: #c19a6b;
    font-size: 1.4em;
    margin-bottom: 15px;
  }
  .note-category ul {
    list-style-type: none;
    padding-left: 0;
  }
  .note-category li {
    margin-bottom: 8px;
  }
  .note-category li::before {
    content: "•";
    color: #d4af37;
    display: inline-block;
    width: 1em;
    margin-left: -1em;
  }
  .stars {
    color: #ffd700;
    font-size: 24px;
    margin-bottom: 10px;
  }
  .half-star {
    color: #ffd700;
    font-size: 24px;
    padding-left: 7px;
    position: absolute;
    display: inline-block;
    overflow: hidden;
    width: 10px;
  }
  .rating-text {
    font-weight: bold;
    color: #d4af37;
  }
  .article-footer {
    text-align: center;
    margin-top: 40px;
    font-size: 0.9em;
    color: #888;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    padding-top: 20px;
  }
  
  .glass-article h2,
  .glass-article h3 {
    pointer-events: none;
    cursor: default;
  }

  .glass-article h2 a,
  .glass-article h3 a {
    color: inherit;
    text-decoration: none;
    pointer-events: none;
    cursor: default;
  }

  .info-grid {
    display: grid;
    margin: 0 auto;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: 10px;
    margin-top: 20px;
  }
  .info-item {
    background-color: rgba(40, 40, 40, 0.7);
    padding: 15px;
    border-radius: 8px;
    text-align: center;
    border: 1px solid rgba(255, 255, 255, 0.05);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    min-height: 100px;
  }
  .info-label {
    font-family: 'Playfair Display', serif;
    color: #c19a6b;
    font-size: 0.9em;
    margin-bottom: 5px;
  }
  .info-value {
    font-family: 'Lato', sans-serif;
    color: #e0e0e0;
    font-size: 1.3em;
    line-height: 1.2;
    flex-grow: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .tasting-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 20px;
    margin-bottom: 30px;
  }

  @media (max-width: 600px) {
    .tasting-grid {
      grid-template-columns: 1fr;
    }

    .info-grid {
      grid-template-columns: repeat(2, 1fr);
    }

    .glass-article {
      padding: 10px;
    }

    .title {
      font-size: 2em;
    }

    .article-image {
      max-width: 100%;
    }
  }

  .note-category {
    background-color: rgba(30, 30, 30, 0.6);
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }
  .note-category h3 {
    color: #d4af37;
    border-bottom: 1px solid #d4af37;
    padding-bottom: 10px;
    margin-bottom: 15px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .score {
    font-size: 0.8em;
    background-color: #d4af37;
    color: #1a1a1a;
    padding: 3px 8px;
    border-radius: 12px;
  }
  .note-category ul {
    list-style-type: none;
    padding-left: 0;
  }
  .note-category li {
    margin-bottom: 8px;
    position: relative;
    padding-left: 20px;
  }
  .note-category li::before {
    content: "•";
    color: #d4af37;
    position: absolute;
    left: 0;
  }
  .rating {
    background-color: rgba(30, 30, 30, 0.6);
    border-radius: 8px;
    padding: 20px;
    text-align: center;
    margin-top: 30px;
  }
  .total-score {
    font-size: 1.2em;
    margin-bottom: 10px;
  }
  .total-score span {
    color: #d4af37;
    font-weight: bold;
  }
  .stars {
    font-size: 24px;
    color: #d4af37;
    margin-bottom: 10px;
  }
  .rating-text {
    font-size: 1.1em;
    font-weight: bold;
  }

  @media (max-width: 600px) {
    .info-grid {
      grid-template-columns: repeat(2, 1fr);
    }
    .info-item {
      min-height: 80px;
    }
    .info-label {
      font-size: 0.8em;
    }
    .info-value {
      font-size: 0.9em;
    }
  }

  @media (max-width: 400px) {
    .info-grid {
      grid-template-columns: repeat(2, 1fr);
    }
    .info-item {
      min-height: 70px;
      padding: 10px;
    }
    .info-label {
      font-size: 0.75em;
    }
    .info-value {
      font-size: 0.85em;
    }
  }
</style>

<article class="glass-article">
  <header class="article-header">
    <h1 class="title">Aultmore 12Y Tasting Notes</h1>
    <img src="/aultmore-12y.png" alt="Aultmore 12Y Bottle" class="article-image">
  </header>

  <section class="tasting-notes">
    <h2 class="section-intro">Information</h2>
    <div class="info-grid">
      <div class="info-item">
        <div class="info-label">Category</div>
        <div class="info-value">Single Malt</div>
      </div>
      <div class="info-item">
        <div class="info-label">Distillery</div>
        <div class="info-value">Aultmore</div>
      </div>
      <div class="info-item">
        <div class="info-label">Bottler</div>
        <div class="info-value">Distillery Bottling</div>
      </div>
      <div class="info-item">
        <div class="info-label">Bottling Serie</div>
        <div class="info-value">Foggie Moss</div>
      </div>
      <div class="info-item">
        <div class="info-label">Cask Type</div>
        <div class="info-value">Refill Hogsheads</div>
      </div>
      <div class="info-item">
        <div class="info-label">Strength</div>
        <div class="info-value">46.0% Vol</div>
      </div>
    </div>
    <h2 class="section-title">Tasting Notes</h2>
    <div class="tasting-grid">
      <div class="note-category appearance">
        <h3>Appearance</h3>
        <p>Clear, with a bright amber hue.</p>
      </div>
      <div class="note-category nose">
        <h3>Nose <span class="score">8/10</span></h3>
        <ul>
          <li>Grassy note with a hint of lemon, green apple</li>
          <li>Subtle floral notes</li>
        </ul>
      </div>
      <div class="note-category palate">
        <h3>Taste <span class="score">7/10</span></h3>
        <ul>
          <li>Persistent hints of herbs, oak</li>
          <li>Balanced, mossy, grassy notes</li>
          <li>Very subtle hits of spice</li>
        </ul>
      </div>
      <div class="note-category finish">
        <h3>Finish <span class="score">8/10</span></h3>
        <p>Medium Length of Finish, the fruit notes fased first with the spices. <br />Honey and the mossy notes lingers on the tip of the tongue, while the oak and herbs fades into a long and pleasant aftertaste.</p>
      </div>
    </div>
    <div class="note-category overall-impression">
      <h3>Overall Impression</h3>
      <p>A well balance between the fruitness and the grassy/mossy notes. <br />Leaving a long and pleasant aftertaste.</p>
    </div>
    <div class="rating">
      <h3>Rating</h3>
      <div class="total-score">Total Score: <span>23/30</span></div>
      <!-- <div class="stars"> -->
        <!-- <span class="star">★</span><span class="star">★</span><span class="star">★</span><span class="half-star">★</span> -->
      <!-- </div> -->
      <p class="rating-text">7.7/10 - Very Good</p>
    </div>
  </section>

</article>