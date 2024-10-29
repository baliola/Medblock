import { useEMRStore } from "@/store/patient-emr";
import { Button, Flex, Icon, Text } from "@chakra-ui/react";
import { usePathname, useSearchParams, useRouter } from "next/navigation";
import { FaChevronLeft, FaChevronRight } from "react-icons/fa";

export default function EMRHistoryPagination() {
  const emrs = useEMRStore(state => state.emrs);

  const searchParams = useSearchParams();
  const pathname = usePathname();
  const router = useRouter();

  const page = Number(searchParams.get("page")) || 0;
  const limit = Number(searchParams.get("limit")) || 10;

  const onPageChange = (page: number) => {
    const param = new URLSearchParams(searchParams);
    param.set("page", page.toString());
    param.set("limit", limit.toString());

    const url = `${pathname}?${param.toString()}`;
    router.push(url);
  }

  const hasNextPage = (): boolean => {
    if (!emrs) return false;
    return emrs.emr.length >= limit
  };

  return (
    <Flex justify={'space-between'} align={'center'} pt={3}>
      <Text fontSize={'xs'}>
        Total EMR: <b>{emrs?.emr.length}</b>
      </Text>
      {emrs && (
        <Flex gap={3}>
          {page > 0 && (
            <Button size={'xs'}
              bg={'primary.200'}
              _hover={{ bg: 'primary.300' }}
              transition={'all 0.2s'}
              fontSize={'xs'}
              leftIcon={
                <Icon as={FaChevronLeft} boxSize={2} />
              }
              onClick={() => onPageChange(page - 1)}
            >
              Previous
            </Button>
          )}
          <Button size={'xs'}
            bg={'primary.200'}
            _hover={{ bg: 'primary.300' }}
            transition={'all 0.2s'}
            fontSize={'xs'}
            rightIcon={
              <Icon as={FaChevronRight} boxSize={2} />
            }
            onClick={() => onPageChange(page + 1)}
            isDisabled={!hasNextPage()}
          >
            Next
          </Button>
        </Flex>
      )}
    </Flex>
  )
}