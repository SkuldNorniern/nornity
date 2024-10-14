import { serverQueryContent } from "#content/server";

export default defineEventHandler(async (event) => {
  console.log("sitemap.xml.ts handler invoked");

  try {
    const URL = "https://nornity.com";

    // Fetch articles
    const articles = await serverQueryContent(event, 'articles')
      .where({ _extension: 'md' })
      .sort({ date: -1 })
      .find();

    console.log(`Fetched ${articles.length} articles for sitemap.`);

    // Start building the sitemap XML
    let sitemap = `<?xml version="1.0" encoding="UTF-8"?>\n`;
    sitemap += `<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n`;

    // Add main page URL
    sitemap += `  <url>\n`;
    sitemap += `    <loc>${URL}/</loc>\n`;
    sitemap += `  </url>\n`;

    // Add static URLs
    const staticUrls = [
      // `${URL}/about`,
      `${URL}/articles`,
      // `${URL}/products`,
    ];

    staticUrls.forEach((path) => {
      sitemap += `  <url>\n`;
      sitemap += `    <loc>${path}</loc>\n`;
      sitemap += `  </url>\n`;
    });

    // Add article URLs
    if (articles && articles.length > 0) {
      for (const article of articles) {
        if (!article.slug) {
          console.warn(`Article missing slug:`, article);
          continue;
        }

        sitemap += `  <url>\n`;
        sitemap += `    <loc>${URL}/articles/${article.slug}</loc>\n`;

        if (article.date) {
          sitemap += `    <lastmod>${new Date(article.date).toISOString()}</lastmod>\n`;
        }

        // Add priority and changefreq if available
        if (article.priority) {
          sitemap += `    <priority>${article.priority}</priority>\n`;
        }
        if (article.changefreq) {
          sitemap += `    <changefreq>${article.changefreq}</changefreq>\n`;
        }

        sitemap += `  </url>\n`;
      }
    } else {
      console.warn("No articles found to include in the sitemap.");
    }

    sitemap += `</urlset>`;

    console.log("Sitemap generation complete.");

    // Set response headers and send sitemap
    event.node.res.setHeader("Content-Type", "application/xml");
    event.node.res.end(sitemap);
  } catch (error) {
    console.error("Error generating sitemap:", error);
    event.node.res.statusCode = 500;
    event.node.res.end("Error generating sitemap");
  }
});