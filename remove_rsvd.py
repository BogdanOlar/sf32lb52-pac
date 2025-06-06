from lxml import etree

# Press Shift+F10 to execute it or replace it with your code.
# Press Double Shift to search everywhere for classes, files, tool windows, actions, and settings.


def print_hi(name):
    # Use a breakpoint in the code line below to debug your script.
    print(f'Hi, {name}')  # Press Ctrl+8 to toggle the breakpoint.


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    # Parse the XML file
    tree = etree.parse('/home/bogdan/Workspace/Projects/sf32lb52-pac/SF32LB52x.svd')
    root = tree.getroot()

    delete_count = 0

    # Find the element you want to remove
    for parent in root.iter():
        for fields in parent.findall('fields'):
            for field in fields.findall('field'):
                for elem in field:
                    if elem.text.startswith('RSVD'):
                        delete_count += 1
                        fields.remove(field)
                        print(delete_count, field.text, elem.text)
                        break

    with open('/home/bogdan/Workspace/Projects/sf32lb52-pac/SF32LB52x_conv.svd', 'wb') as f:
        f.write(etree.tostring(root, pretty_print=True))
