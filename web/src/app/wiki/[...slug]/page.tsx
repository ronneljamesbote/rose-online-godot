import type { Metadata } from "next";
import { loadWiki } from "@/lib/wiki";
import WikiPageView from "../WikiPageView";

export const dynamic = "force-dynamic";

type Props = { params: Promise<{ slug: string[] }> };

function pathOf(slug: string[]): string {
  return slug.map((s) => decodeURIComponent(s)).join("/");
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const path = pathOf((await params).slug);
  const page = loadWiki().pages.get(path);
  return { title: `${page?.title ?? path} · ROSE wiki` };
}

export default async function WikiPage({ params }: Props) {
  return <WikiPageView path={pathOf((await params).slug)} />;
}
