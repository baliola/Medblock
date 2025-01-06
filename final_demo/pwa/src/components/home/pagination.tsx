import { homePaginationButton } from "@/constants/contents/home/button";
import { useEMRStore } from "@/store/emr-store";
import { Button, Flex, Icon, Text } from "@chakra-ui/react";
import { usePathname, useRouter, useSearchParams } from "next/navigation";

export default function HomePagination() {
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

  return (
    <Flex justify={'space-between'} align={'center'}>
      <Text fontSize={'sm'}>
        {`${homePaginationButton.total.label} ${emrs.length}`}
      </Text>
      <Flex gap={3}>
        {page > 0 && (
          <Button size={'sm'}
            leftIcon={
              <Icon
                as={homePaginationButton.previous.icon}
                boxSize={3}
              />
            }
            onClick={() => onPageChange(page - 1)}
          >
            {homePaginationButton.previous.label}
          </Button>
        )}
        <Button size={'sm'}
          rightIcon={
            <Icon
              as={homePaginationButton.next.icon}
              boxSize={3}
            />
          }
          onClick={() => onPageChange(page + 1)}
        >
          {homePaginationButton.next.label}
        </Button>
      </Flex>
    </Flex>
  )
}