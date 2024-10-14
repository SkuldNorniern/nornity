import { serverQueryContent } from "#content/server";

export default defineEventHandler(async (event) => {
  try {
    const URL = "https://nornity.com";

    // Fetch articles
    const articles = await serverQueryContent(event, 'articles')
      .where({ _extension: 'md' })
      .sort({ date: -1 })
      .find();

    // Start building the sitemap XML
    let sitemap = `<?xml version="1.0" encoding="UTF-8"?>\n`;
    sitemap += `<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n`;

    // Add main page URL
    sitemap += `  <url>\n`;
    sitemap += `    <loc>${URL}/</loc>\n`;
    sitemap += `  </url>\n`;

    // Add article URLs
    for (const article of articles) {
      sitemap += `  <url>\n`;
      sitemap += `    <loc>${URL}/articles/${article.slug}</loc>\n`;
      if (article.date) {
        sitemap += `    <lastmod>${new Date(article.date).toISOString()}</lastmod>\n`;
      }
      sitemap += `  </url>\n`;
    }

    sitemap += `</urlset>`;

    // Set response headers and send sitemap
    event.node.res.setHeader("Content-Type", "application/xml");
    event.node.res.end(sitemap);
  } catch (error) {
    console.error("Error generating sitemap:", error);
    return { statusCode: 500, body: "Error generating sitemap" };
  }
});

