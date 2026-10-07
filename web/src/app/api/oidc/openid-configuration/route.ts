// OpenID discovery, so the game server can find the keys that sign game tokens.

export const dynamic = "force-dynamic";

import { AUTH_ISSUER } from "@/lib/config";

export function GET() {
  return Response.json({
    issuer: AUTH_ISSUER,
    jwks_uri: `${AUTH_ISSUER}/.well-known/jwks.json`,
    id_token_signing_alg_values_supported: ["ES256"],
    subject_types_supported: ["public"],
    response_types_supported: ["id_token"],
  });
}
