SELECT deck_id,
  sum(queue = :new_queue),
  sum(queue = :learn_queue),
  sum(queue = :review_queue),
  COUNT(1)
FROM cards
GROUP BY deck_id
