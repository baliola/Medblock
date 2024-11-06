import HAMPageContent from "@/components/ham";
import HAMDetailHospital from "@/components/ham/detail";
import { Flex } from "@chakra-ui/react";

interface PageProps {
  searchParams: {
    currentPage?: number;
    limit?: number;
    hospital?: string;
  };
}

export default async function HAMPage({ searchParams }: PageProps) {
  const provider = searchParams.hospital || null;

  return (
    <Flex w={"full"} flex={1}>
      <HAMPageContent />
      {provider && <HAMDetailHospital />}
    </Flex>
  );
}
