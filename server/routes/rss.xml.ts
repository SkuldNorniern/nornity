import RSS from "rss";
import { serverQueryContent } from "#content/server";

export default defineEventHandler(async (event) => {
  try {
    const URL = process.env.PUBLIC_URL?.toString() as string;
    const TITLE = process.env.PUBLIC_TITLE?.toString() as string;
    const DESCRIPTION = process.env.PUBLIC_DESCRIPTION?.toString() as string;

    // Query articles based on the structure in index.vue
    const articles = await serverQueryContent(event, 'articles')
      .where({ _extension: 'md' })
      .sort({ date: -1 })
      .find();

    const feed = new RSS({
      title: TITLE,
      description: DESCRIPTION,
      site_url: URL,
      feed_url: `${URL}/rss.xml`,
      language: "en",
      ttl: 60,
    });

    for (const article of articles) {
      feed.item({
        title: article.title?.toString() as string,
        url: `${URL}/articles/${article.slug}`,
        date: new Date(article.date),
        description: article.description,
        // You can add more fields here if needed
      });
    }

    const feedString = feed.xml({ indent: true });

    event.node.res.setHeader("content-type", "text/xml");
    event.node.res.end(feedString);
  } catch (e) {
    console.error("Error generating RSS feed:", e);
    return { statusCode: 500, body: "Error generating RSS feed" };
  }
});
