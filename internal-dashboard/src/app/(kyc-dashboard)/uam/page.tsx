import UAMPageData from "@/components/uam";

interface PageProps {
  searchParams: {
    currentPage?: number;
    limit?: number;
    user?: string;
  }
}

export default async function UAMPage({ searchParams }: PageProps) {
  const user = searchParams.user || null;

  return (
    <UAMPageData user={user} />
  )
}