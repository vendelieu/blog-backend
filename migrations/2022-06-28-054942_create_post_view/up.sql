CREATE VIEW post_view AS
SELECT id,
       image,
       title,
       content,
       description,
       slug,
       commentaries_open,
       (select array_agg((tags.id, name, tags.slug)::tag)
        from tags
        where tags.slug in (select post_tags_pivot.tag_slug from post_tags_pivot where post_slug = posts.slug)
        limit 5)                                                                                         as "tags",
       (select (p.title, p.slug)::nav_post from posts as p where id < posts.id order by id desc limit 1) as "prev",
       (select (p.title, p.slug)::nav_post from posts as p where id > posts.id order by id limit 1)      as "next",
       posts.updated_at
from posts;