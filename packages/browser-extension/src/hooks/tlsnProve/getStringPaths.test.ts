import { getStringPaths } from "./getStringPaths";
import { describe, test, expect } from "vitest";
describe("getStringPaths", () => {
  test("empty object", () => {
    expect(getStringPaths("{}")).toEqual([]);
  });

  test("flat object", () => {
    const input = JSON.stringify({
      name: "John",
      age: 30,
      city: "New York",
    });
    expect(getStringPaths(input)).toEqual(["name", "city"]);
  });

  test(" nested string paths", () => {
    const input = JSON.stringify({
      user: {
        name: "John",
        address: {
          street: "123 Main St",
          city: "New York",
        },
      },
    });
    expect(getStringPaths(input)).toEqual([
      "user.name",
      "user.address.street",
      "user.address.city",
    ]);
  });

  test("array", () => {
    const input = JSON.stringify({
      users: [{ name: "John" }, { name: "Jane" }],
    });
    expect(getStringPaths(input)).toEqual(["users.0.name", "users.1.name"]);
  });

  test("deeply nested arrays and objects", () => {
    const input = JSON.stringify({
      organization: {
        departments: [
          {
            name: "Engineering",
            teams: [
              {
                lead: {
                  name: "Alice",
                  age: 30,
                  contact: {
                    email: "alice@example.com",
                    phone: "123-456-7890",
                  },
                },
              },
            ],
          },
        ],
      },
    });
    expect(getStringPaths(input)).toEqual([
      "organization.departments.0.name",
      "organization.departments.0.teams.0.lead.name",
      "organization.departments.0.teams.0.lead.contact.email",
      "organization.departments.0.teams.0.lead.contact.phone",
    ]);
  });
});
