import type { GetSession, Handle } from "@sveltejs/kit";
import { parse } from "cookie";

export const handle: Handle = async ({ resolve, event }) => {
  const cookies = parse(event.request.headers.get("cookie") || "");
  if (cookies.authtoken) {
    event.locals.authtoken = cookies.authtoken;
  }

  const response = await resolve(event);
  return response;
};

export const getSession: GetSession = (event) => {
  return {
    authtoken: event.locals.authtoken ? event.locals.authtoken : null,
  };
};
