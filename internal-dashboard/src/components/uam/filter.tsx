import { Button, Flex, Icon, Menu, MenuButton, MenuGroup, MenuList, Radio, RadioGroup, Stack, Text } from "@chakra-ui/react";
import { BiFilter } from "react-icons/bi";
import { FaX } from "react-icons/fa6";

const filterOptions = [
  { name: "All", value: "all", color: "gray" },
  { name: "Pending", value: "pending", color: "yellow.200" },
  { name: "Accepted", value: "accepted", color: "green.200" },
  { name: "Rejected", value: "rejected", color: "red.200" },
] as const;

interface FilterMenuProps {
  selectedStatus: string;
  setSelectedStatus: (status: string) => void;
}

export const FilterMenu = ({
  selectedStatus, setSelectedStatus
}: FilterMenuProps) => {
  return (
    <Menu closeOnSelect={false}>
      <MenuButton as={Button} size={'lg'} rounded={'xl'} p={2}>
        <Icon as={BiFilter} boxSize={5} />
      </MenuButton>
      <MenuList>
        <MenuGroup title="Status" fontSize={'md'}>
          <RadioGroup onChange={setSelectedStatus} value={selectedStatus}>
            <Stack px={5}>
              {filterOptions.map((item) => (
                <Radio key={item.value} value={item.value}>
                  {item.name}
                </Radio>
              ))}
            </Stack>
          </RadioGroup>
        </MenuGroup>
      </MenuList>
    </Menu>
  )
}

interface FilterTagsProps {
  selectedStatus: string;
  onClearFilter: () => void;
}

export const FilterTags = ({
  selectedStatus, onClearFilter
}: FilterTagsProps) => {
  return (
    <Flex
      bg={
        filterOptions.find(item => item.value === selectedStatus)?.color
      }
      fontSize={'sm'}
      px={3} py={1}
      rounded={'md'}
      w={'fit-content'}
      gap={2}
      align={'center'}
      cursor={'pointer'}
      onClick={onClearFilter}
    >
      <Text>
        {filterOptions.find(item => item.value === selectedStatus)?.name}
      </Text>
      <Icon as={FaX} boxSize={2} />
    </Flex>
  )
}