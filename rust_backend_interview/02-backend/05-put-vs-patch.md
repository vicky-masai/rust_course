# PUT vs PATCH

## Interview Question

PUT vs PATCH.

## Interview Answer

"PUT replaces the entire resource. PATCH updates only specified fields."

---

## Follow-up Questions & Answers

### Q1. When would you choose PUT over PATCH in a real API?

**Interview Answer**

I use PUT when the client sends the complete updated resource and the server should replace it entirely, like updating a user profile where all fields are mandatory. PATCH is better when the client only wants to change one or two fields, such as updating just an email address without resending the entire object. PATCH reduces bandwidth and avoids accidental overwrites of unchanged fields.

---

### Q2. How do you implement PATCH safely to avoid partial update issues?

**Interview Answer**

I use partial deserialization with `Option<T>` fields in the request struct so missing fields remain `None` and are not overwritten in the database. The handler merges the provided fields with the existing record using a SQL `UPDATE ... SET` statement that only touches non-None columns. This prevents sending null values that would unintentionally clear database fields.

---

### Q3. What is the idempotency difference between PUT and PATCH?

**Interview Answer**

PUT is idempotent by specification, meaning calling it multiple times with the same body produces the same result. PATCH is not inherently idempotent because a partial update applied twice could produce different results if the operation involves appending or incrementing. However, a well-designed PATCH using `replace` operations can be made idempotent.

---

### Q4. How do you handle PUT requests when the resource does not exist?

**Interview Answer**

I treat a PUT to a non-existent resource as a create operation and return `201 Created` instead of `404 Not Found`. This follows the REST convention where PUT means "ensure this resource exists with this data." Some APIs prefer returning `404` and requiring a separate POST for creation, so consistency with your API contract matters.

---

### Q5. What is JSON Merge Patch and how does it differ from JSON Patch?

**Interview Answer**

JSON Merge Patch (RFC 7386) lets you send a partial object where `null` fields mean "remove this field." JSON Patch (RFC 6902) uses an array of operations like `add`, `remove`, `replace`, `move`, and `copy` for more granular control. I prefer Merge Patch for simple updates and JSON Patch when I need complex multi-field operations in a single request.

---

### Q6. How would you implement PATCH in Axum with sqlx?

**Interview Answer**

I define a `UpdateUser` struct with `Option<String>` fields, deserialize it from the JSON body, and build a dynamic SQL query using `sqlx::QueryBuilder`. Each non-None field gets added to the `SET` clause with a bind parameter. This approach avoids writing separate queries for every possible field combination and keeps the code maintainable.

---

### Q7. Should PATCH requests be atomic?

**Interview Answer**

Yes, a PATCH request should be atomic so either all specified fields are updated or none are. I wrap the database update inside a transaction so partial failures roll back cleanly. If the PATCH touches multiple tables, the transaction ensures consistency across all affected resources.

---
