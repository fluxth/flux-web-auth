import type { GetSession, Handle } from "@sveltejs/kit";
import { parse, serialize } from "cookie";
import { decode } from "jsonwebtoken";

const AUTH_COOKIE_NAME = import.meta.env.VITE_AUTH_COOKIE_NAME;
const AUTH_COOKIE_DOMAIN = import.meta.env.VITE_AUTH_COOKIE_DOMAIN;
const AUTH_TOKEN_ISSUER = "auth.flux.ci";

export const handle: Handle = async ({ resolve, event }) => {
  let clearAuthCookie = false;

  const cookies = parse(event.request.headers.get("cookie") || "");
  if (cookies[AUTH_COOKIE_NAME]) {
    const token = cookies[AUTH_COOKIE_NAME];
    const decoded = decode(token, { json: true });

    // Try check if JWT is decodable, and if the issuer is this app.
    // This *doesn't* verify the JWT signature!
    if (decoded?.iss === AUTH_TOKEN_ISSUER) event.locals.authtoken = token;
    else clearAuthCookie = true;
  }

  const response = await resolve(event);

  if (clearAuthCookie) {
    response.headers.append(
      "Set-Cookie",
      serialize(AUTH_COOKIE_NAME, "", {
        domain: AUTH_COOKIE_DOMAIN,
        path: "/",
        expires: new Date(0),
      })
    );
  }

  return response;
};

export const getSession: GetSession = (event) => {
  return {
    authtoken: event.locals.authtoken ? event.locals.authtoken : null,
  };
};
