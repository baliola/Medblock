import AdminPageContent from "@/components/admin";
import { Flex } from "@chakra-ui/react";

interface PageProps {
  searchParams: {
    currentPage?: number;
    limit?: number;
    admin?: string;
  };
}

export default async function AdminPage({ }: PageProps) {
  // const provider = searchParams.admin || null;

  return (
    <Flex w={"full"} flex={1}>
      <AdminPageContent />
      {/* {provider && <HAMDetailHospital />} */}
    </Flex>
  );
}
