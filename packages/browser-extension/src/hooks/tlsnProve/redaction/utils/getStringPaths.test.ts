import { getStringPaths } from "./getStringPaths";
import { describe, test, expect } from "vitest";
import { Utf8String } from "./utf8String";
describe("getStringPaths", () => {
  test("empty object", () => {
    expect(getStringPaths(new Utf8String("{}"))).toEqual([]);
  });

  test("flat object", () => {
    const input = new Utf8String(
      JSON.stringify({
        name: "José",
        age: 30,
        city: "São Paulo",
      }),
    );
    expect(getStringPaths(input)).toEqual(["name", "city"]);
  });

  test("nested string paths", () => {
    const input = new Utf8String(
      JSON.stringify({
        user: {
          name: "François",
          address: {
            street: "123 Champs-Élysées",
            city: "Montréal",
          },
        },
      }),
    );
    expect(getStringPaths(input)).toEqual([
      "user.name",
      "user.address.street",
      "user.address.city",
    ]);
  });

  test("array", () => {
    const input = new Utf8String(
      JSON.stringify({
        users: [{ name: "Jürgen" }, { name: "Zoë" }],
      }),
    );
    expect(getStringPaths(input)).toEqual(["users.0.name", "users.1.name"]);
  });

  test("deeply nested arrays and objects", () => {
    const input = new Utf8String(
      JSON.stringify({
        organization: {
          departments: [
            {
              name: "Développement",
              teams: [
                {
                  lead: {
                    name: "Amélie",
                    age: 30,
                    contact: {
                      email: "amélie@example.com",
                      phone: "123-456-7890",
                    },
                  },
                },
              ],
            },
          ],
        },
      }),
    );
    expect(getStringPaths(input)).toEqual([
      "organization.departments.0.name",
      "organization.departments.0.teams.0.lead.name",
      "organization.departments.0.teams.0.lead.contact.email",
      "organization.departments.0.teams.0.lead.contact.phone",
    ]);
  });
});
